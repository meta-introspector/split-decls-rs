macro_rules! macro_437 {
    () => {
        pin_project ! { # [doc = " Stream for the [`take`](super::StreamExt::take) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Take < St > { # [pin] stream : St , remaining : usize , } }
    };
}

macro_437!();