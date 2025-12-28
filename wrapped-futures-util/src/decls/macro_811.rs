macro_rules! macro_811 {
    () => {
        pin_project ! { # [doc = " Stream for the [`unfold`] function."] # [must_use = "streams do nothing unless polled"] pub struct Unfold < T , F , Fut > { f : F , # [pin] state : UnfoldState < T , Fut >, } }
    };
}

macro_811!();