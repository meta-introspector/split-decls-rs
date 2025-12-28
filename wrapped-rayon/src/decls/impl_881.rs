macro_rules! deps {
    () => {
        TakeAnyConsumer!();
        Reducer!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_881 {
    () => {
        deps!();
        impl < 'f , T , C > UnindexedConsumer < T > for TakeAnyConsumer < 'f , C > where C : UnindexedConsumer < T > , T : Send , { fn split_off_left (& self) -> Self { TakeAnyConsumer { base : self . base . split_off_left () , .. * self } } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_881!()