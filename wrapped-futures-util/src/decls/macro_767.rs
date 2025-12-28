macro_rules! macro_767 {
    () => {
        pin_project ! { # [doc = " A stream which emits single element and then EOF."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Once < Fut > { # [pin] future : Option < Fut > } }
    };
}

macro_767!();