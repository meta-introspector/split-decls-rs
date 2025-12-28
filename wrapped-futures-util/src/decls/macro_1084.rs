macro_rules! macro_1084 {
    () => {
        pin_project ! { # [doc = " Reader for the [`chain`](super::AsyncReadExt::chain) method."] # [must_use = "readers do nothing unless polled"] pub struct Chain < T , U > { # [pin] first : T , # [pin] second : U , done_first : bool , } }
    };
}

macro_1084!();