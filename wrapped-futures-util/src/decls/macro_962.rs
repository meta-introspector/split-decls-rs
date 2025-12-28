macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! macro_962 {
    () => {
        deps!();
        pin_project ! { # [doc = " Sink for the [`unfold`] function."] # [derive (Debug)] # [must_use = "sinks do nothing unless polled"] pub struct Unfold < T , F , Fut > { function : F , # [pin] state : UnfoldState < T , Fut >, } }
    };
}

macro_962!();