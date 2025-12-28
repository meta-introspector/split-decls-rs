macro_rules! deps {
    () => {
        UnindexedConsumer!();
        Reducer!();
        TryReduceConsumer!();
    };
}

macro_rules! impl_919 {
    () => {
        deps!();
        impl < 'r , R , ID , T > UnindexedConsumer < T > for TryReduceConsumer < 'r , R , ID > where R : Fn (T :: Output , T :: Output) -> T + Sync , ID : Fn () -> T :: Output + Sync , T : Try + Send , { fn split_off_left (& self) -> Self { * self } fn to_reducer (& self) -> Self :: Reducer { * self } }
    };
}

impl_919!()