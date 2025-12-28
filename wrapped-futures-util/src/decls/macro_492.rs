macro_rules! macro_492 {
    () => {
        pin_project ! { # [doc = " Stream for the [`scan`](super::StreamExt::scan) method."] # [must_use = "streams do nothing unless polled"] pub struct Scan < St : Stream , S , Fut , F > { # [pin] stream : St , f : F , # [pin] state : UnfoldState < S , Fut >, } }
    };
}

macro_492!();