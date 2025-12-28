macro_rules! macro_138 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::skip()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Skip < S > { # [pin] stream : S , n : usize , } }
    };
}

macro_138!();