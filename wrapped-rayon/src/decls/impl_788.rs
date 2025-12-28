macro_rules! deps {
    () => {
        ReduceConsumer!();
    };
}

macro_rules! impl_788 {
    () => {
        deps!();
        impl < 'r , R , ID > Clone for ReduceConsumer < 'r , R , ID > { fn clone (& self) -> Self { * self } }
    };
}

impl_788!();