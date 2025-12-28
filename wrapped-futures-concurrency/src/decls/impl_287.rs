macro_rules! deps {
    () => {
        AggregateError!();
    };
}

macro_rules! impl_287 {
    () => {
        deps!();
        impl < E , const N : usize > AggregateError < E , N > { pub (super) fn new (inner : [E ; N]) -> Self { Self { inner } } }
    };
}

impl_287!()