macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! macro_982 {
    () => {
        deps!();
        pin_project ! { # [doc = " Sink for the [`buffer`](super::SinkExt::buffer) method."] # [derive (Debug)] # [must_use = "sinks do nothing unless polled"] pub struct Buffer < Si , Item > { # [pin] sink : Si , buf : VecDeque < Item >, capacity : usize , } }
    };
}

macro_982!()