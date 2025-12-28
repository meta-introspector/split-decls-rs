macro_rules! macro_150 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::cycle()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Cycle < S > { orig : S , # [pin] stream : S , } }
    };
}

macro_150!();