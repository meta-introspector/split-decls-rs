macro_rules! macro_312 {
    () => {
        pin_project ! { # [doc = " Stream for the [`enumerate`](super::StreamExt::enumerate) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Enumerate < St > { # [pin] stream : St , count : usize , } }
    };
}

macro_312!();