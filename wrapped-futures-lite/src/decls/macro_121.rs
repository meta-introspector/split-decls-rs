macro_rules! macro_121 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::filter()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Filter < S , P > { # [pin] stream : S , predicate : P , } }
    };
}

macro_121!();