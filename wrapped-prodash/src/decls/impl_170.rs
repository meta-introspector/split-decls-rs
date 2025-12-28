macro_rules! deps {
    () => {
        NestedProgress!();
        ThroughputOnDrop!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < T : NestedProgress > ThroughputOnDrop < T > { # [doc = " Create a new instance by providing the `inner` [`NestedProgress`] implementation."] pub fn new (inner : T) -> Self { ThroughputOnDrop (inner , Instant :: now ()) } }
    };
}

impl_170!()