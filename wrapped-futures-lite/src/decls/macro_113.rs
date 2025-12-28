macro_rules! macro_113 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::map()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Map < S , F > { # [pin] stream : S , f : F , } }
    };
}

macro_113!();