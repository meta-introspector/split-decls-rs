macro_rules! macro_424 {
    () => {
        pin_project ! { # [doc = " Stream for the [`skip`](super::StreamExt::skip) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Skip < St > { # [pin] stream : St , remaining : usize , } }
    };
}

macro_424!()