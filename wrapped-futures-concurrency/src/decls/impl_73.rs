macro_rules! deps {
    () => {
        ReadinessArray!();
        InlineWakerArray!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < const N : usize > InlineWakerArray < N > { # [doc = " Create a new instance of `InlineWaker`."] pub (crate) fn new (id : usize , readiness : Arc < Mutex < ReadinessArray < N > > >) -> Self { Self { id , readiness } } }
    };
}

impl_73!();