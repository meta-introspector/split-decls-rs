macro_rules! deps {
    () => {
        Reducer!();
        ReduceConsumer!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_790 {
    () => {
        deps!();
        impl < 'r , R , ID , T > UnindexedConsumer < T > for ReduceConsumer < 'r , R , ID > where R : Fn (T , T) -> T + Sync , ID : Fn () -> T + Sync , T : Send , { fn split_off_left (& self) -> Self { * self } fn to_reducer (& self) -> Self :: Reducer { * self } }
    };
}

impl_790!();