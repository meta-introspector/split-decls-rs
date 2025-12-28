macro_rules! deps {
    () => {
        AggregateError!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl < E > AggregateError < E > { pub (crate) fn new (inner : Vec < E >) -> Self { Self { inner } } }
    };
}

impl_327!()