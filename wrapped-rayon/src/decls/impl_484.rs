macro_rules! deps {
    () => {
        Reducer!();
        FindReducer!();
        FindConsumer!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_484 {
    () => {
        deps!();
        impl < 'p , T , P : 'p > UnindexedConsumer < T > for FindConsumer < 'p , P > where T : Send , P : Fn (& T) -> bool + Sync , { fn split_off_left (& self) -> Self { FindConsumer :: new (self . find_op , self . found) } fn to_reducer (& self) -> Self :: Reducer { FindReducer } }
    };
}

impl_484!()