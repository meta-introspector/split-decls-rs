macro_rules! macro_132 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::take()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Take < S > { # [pin] stream : S , n : usize , } }
    };
}

macro_132!();