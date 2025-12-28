macro_rules! macro_76 {
    () => {
        pin_project ! { # [doc = " Stream for the [`try_unfold()`] function."] # [derive (Clone)] # [must_use = "streams do nothing unless polled"] pub struct TryUnfold < T , F , Fut > { f : F , state : Option < T >, # [pin] fut : Option < Fut >, } }
    };
}

macro_76!();