macro_rules! deps {
    () => {
        InlineWakerVec!();
        ReadinessVec!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl InlineWakerVec { # [doc = " Create a new instance of `InlineWaker`."] pub (crate) fn new (id : usize , readiness : Arc < Mutex < ReadinessVec > >) -> Self { Self { id , readiness } } }
    };
}

impl_97!()