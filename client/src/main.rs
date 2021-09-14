use yew::prelude::*;

enum Msg {
    ClickConnect,
    SendMsg,
    InputMsg(String),
}

enum ConnectStatus {
    Connected,
    Disconnected,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    status: ConnectStatus,
    logs: Vec<String>,
    message: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            status: ConnectStatus::Disconnected,
            logs: Vec::new(),
            message: String::from(""),

        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ClickConnect => {
                match self.status {
                    ConnectStatus::Connected => {
                        self.status = ConnectStatus::Disconnected;
                        self.logs.push(String::from("disconnected"));
                    },
                    ConnectStatus::Disconnected => {
                        self.logs.push(String::from("connected"));
                        self.status = ConnectStatus::Connected;
                    },
                }
                true
            },
            Msg::SendMsg => {
                self.logs.push(String::from(format!("Sending {}", self.message)));
                self.message = String::new();
                true
            },
            Msg::InputMsg(s) => {
                self.message = s;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
           <div>
               <div>{self.header()}</div>
               <div>{self.log_view()}</div>
               <div>{self.message_form()}</div>
           </div>
        }
    }
}

impl Model {
    fn header(&self) -> Html {
        let connect_button = match self.status {
            ConnectStatus::Connected => "Disconnected",
            ConnectStatus::Disconnected => "Connected",
        };
        let status = match self.status {
            ConnectStatus::Connected => "Connected()",
            ConnectStatus::Disconnected => "Disconnected",
        };
        html! {
            <div>
                <h3>{ "Chat! "}</h3>
                <button onclick={ self.link.callback(|_| Msg::ClickConnect)}>{connect_button}</button>
                <span>{status}</span>
            </div>
        }
    }

    fn log_view(&self) -> Html {
        html! {
            <div style="width:20em;height:15em;overflow:auto;border:1px solid black">{ self.logs.iter().map(|log| html! {<div>{log}</div> }).collect::<Vec<_>>() }</div>
        }
    }

    fn message_form(&self) -> Html {
        html! {
            <div>
                <input value={ self.message.clone() } oninput={ self.link.callback(|e: InputData| Msg::InputMsg(e.value.clone())) } type="text" />
                <button onclick={ self.link.callback(|_| Msg::SendMsg )}>{"Send"}</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
