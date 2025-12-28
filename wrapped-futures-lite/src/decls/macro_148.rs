macro_rules! macro_148 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::copied()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Copied < S > { # [pin] stream : S , } }
    };
}

macro_148!();