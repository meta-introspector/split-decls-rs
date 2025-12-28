macro_rules! macro_52 {
    () => {
        pin_project ! { # [doc = " Stream for the [`once()`] function."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Once < T > { value : Option < T >, } }
    };
}

macro_52!()