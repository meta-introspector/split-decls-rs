macro_rules! macro_281 {
    () => {
        pin_project ! { # [doc = " Stream for the [`chain`](super::StreamExt::chain) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Chain < St1 , St2 > { # [pin] first : Option < St1 >, # [pin] second : St2 , } }
    };
}

macro_281!();