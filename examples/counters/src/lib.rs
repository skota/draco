use self::counter::Counter;
use wasm_bindgen::prelude::*;

pub mod counter {
    #[derive(Default)]
    pub struct Counter {
        value: i32,
    }

    pub enum Message {
        Increment,
        Decrement,
        Reset,
        Remove,
    }

    impl Counter {
        pub fn update(&mut self, message: Message) {
            match message {
                Message::Increment => self.value += 1,
                Message::Decrement => self.value -= 1,
                Message::Reset => self.value = 0,
                Message::Remove => {}
            }
        }

        pub fn view(&self) -> draco::VNode<Message> {
            use draco::html as h;
            h::div()
                .with((
                    h::button().with("-").on("click", |_| Message::Decrement),
                    " ",
                    self.value,
                    " ",
                    h::button().with("+").on("click", |_| Message::Increment),
                    " ",
                    h::button().with("Reset").on("click", |_| Message::Reset),
                    " ",
                    h::button().with("Remove").on("click", |_| Message::Remove),
                ))
                .into()
        }
    }
}

#[derive(Default)]
pub struct Counters {
    counters: Vec<Counter>,
}

pub enum Message {
    Append,
    Counter(usize, counter::Message),
}

impl draco::Application for Counters {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _mailbox: &draco::Mailbox<Self::Message>) {
        match message {
            Message::Append => self.counters.push(Counter::default()),
            Message::Counter(index, counter::Message::Remove) => {
                self.counters.remove(index);
            }
            Message::Counter(index, message) => {
                self.counters[index].update(message);
            }
        }
    }

    fn view(&self) -> draco::VNode<Self::Message> {
        use draco::html as h;
        h::div()
            .with(h::button().with("Append").on("click", |_| Message::Append))
            .append(self.counters.iter().enumerate().map(|(index, counter)| {
                counter
                    .view()
                    .map(move |message| Message::Counter(index, message))
            }))
            .into()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    draco::start(
        Counters::default(),
        draco::select("main").expect("<main>").into(),
    );
}
