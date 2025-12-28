macro_rules! deps {
    () => {
        TryReduceWithConsumer!();
    };
}

macro_rules! impl_927 {
    () => {
        deps!();
        impl < 'r , R > Clone for TryReduceWithConsumer < 'r , R > { fn clone (& self) -> Self { * self } }
    };
}

impl_927!()