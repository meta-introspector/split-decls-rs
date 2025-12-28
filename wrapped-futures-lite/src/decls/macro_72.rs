macro_rules! macro_72 {
    () => {
        pin_project ! { # [doc = " Stream for the [`unfold()`] function."] # [derive (Clone)] # [must_use = "streams do nothing unless polled"] pub struct Unfold < T , F , Fut > { f : F , state : Option < T >, # [pin] fut : Option < Fut >, } }
    };
}

macro_72!();