macro_rules! macro_134 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::take_while()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct TakeWhile < S , P > { # [pin] stream : S , predicate : P , } }
    };
}

macro_134!()