macro_rules! macro_450 {
    () => {
        pin_project ! { # [doc = " Stream for the [`take_until`](super::StreamExt::take_until) method."] # [must_use = "streams do nothing unless polled"] pub struct TakeUntil < St : Stream , Fut : Future > { # [pin] stream : St , # [pin] fut : Option < Fut >, fut_result : Option < Fut :: Output >, free : bool , } }
    };
}

macro_450!();