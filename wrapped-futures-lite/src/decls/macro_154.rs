macro_rules! macro_154 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::inspect()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Inspect < S , F > { # [pin] stream : S , f : F , } }
    };
}

macro_154!()