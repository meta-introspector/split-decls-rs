macro_rules! deps {
    () => {
        UnindexedConsumer!();
        TryReduceWithConsumer!();
        Reducer!();
    };
}

macro_rules! impl_929 {
    () => {
        deps!();
        impl < 'r , R , T > UnindexedConsumer < T > for TryReduceWithConsumer < 'r , R > where R : Fn (T :: Output , T :: Output) -> T + Sync , T : Try + Send , { fn split_off_left (& self) -> Self { * self } fn to_reducer (& self) -> Self :: Reducer { * self } }
    };
}

impl_929!()