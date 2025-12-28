macro_rules! macro_332 {
    () => {
        pin_project ! { # [doc = " Stream for the [`flatten`](super::StreamExt::flatten) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Flatten < St , U > { # [pin] stream : St , # [pin] next : Option < U >, } }
    };
}

macro_332!()