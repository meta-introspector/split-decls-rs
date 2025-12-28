macro_rules! impl_533 {
    () => {
        impl < St > PollStreamFut < St > { # [doc = " Constructs new `PollStreamFut` using given `stream`."] fn new (stream : impl Into < Option < St > >) -> Self { Self { stream : stream . into () } } }
    };
}

impl_533!();