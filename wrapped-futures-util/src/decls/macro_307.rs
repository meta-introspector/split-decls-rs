macro_rules! macro_307 {
    () => {
        pin_project ! { # [doc = " Stream for the [`cycle`](super::StreamExt::cycle) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Cycle < St > { orig : St , # [pin] stream : St , } }
    };
}

macro_307!()