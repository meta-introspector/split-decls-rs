macro_rules! macro_146 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::cloned()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Cloned < S > { # [pin] stream : S , } }
    };
}

macro_146!();