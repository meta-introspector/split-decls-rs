macro_rules! macro_117 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::flatten()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Flatten < S : Stream > { # [pin] stream : S , # [pin] inner_stream : Option < S :: Item >, } }
    };
}

macro_117!();