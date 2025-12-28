macro_rules! macro_152 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::enumerate()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Enumerate < S > { # [pin] stream : S , i : usize , } }
    };
}

macro_152!()