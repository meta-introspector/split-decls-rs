macro_rules! macro_83 {
    () => {
        pin_project ! { # [doc = " Stream for the [`stop_after_future()`] function."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct StopAfterFuture < S : Stream , Fut : Future > { # [pin] stream : S , # [pin] fut : Option < Fut >, fut_result : Option < Fut :: Output >, free : bool , } }
    };
}

macro_83!();