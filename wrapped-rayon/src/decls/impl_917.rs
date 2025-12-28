macro_rules! deps {
    () => {
        TryReduceConsumer!();
    };
}

macro_rules! impl_917 {
    () => {
        deps!();
        impl < 'r , R , ID > Clone for TryReduceConsumer < 'r , R , ID > { fn clone (& self) -> Self { * self } }
    };
}

impl_917!();