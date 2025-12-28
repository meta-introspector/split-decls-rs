macro_rules! deps {
    () => {
        UnindexedConsumer!();
        SkipAnyConsumer!();
        Reducer!();
    };
}

macro_rules! impl_830 {
    () => {
        deps!();
        impl < 'f , T , C > UnindexedConsumer < T > for SkipAnyConsumer < 'f , C > where C : UnindexedConsumer < T > , T : Send , { fn split_off_left (& self) -> Self { SkipAnyConsumer { base : self . base . split_off_left () , .. * self } } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_830!()