macro_rules! macro_111 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::fuse()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Fuse < S > { # [pin] stream : S , done : bool , } }
    };
}

macro_111!();