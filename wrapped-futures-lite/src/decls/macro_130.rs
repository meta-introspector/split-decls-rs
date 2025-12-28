macro_rules! macro_130 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::filter_map()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct FilterMap < S , F > { # [pin] stream : S , f : F , } }
    };
}

macro_130!();