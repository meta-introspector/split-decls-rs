macro_rules! macro_691 {
    () => {
        pin_project ! { # [doc = " Stream for the [`try_unfold`] function."] # [must_use = "streams do nothing unless polled"] pub struct TryUnfold < T , F , Fut > { f : F , state : Option < T >, # [pin] fut : Option < Fut >, } }
    };
}

macro_691!();