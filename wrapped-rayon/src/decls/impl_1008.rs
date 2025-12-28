macro_rules! deps {
    () => {
        WhileSomeConsumer!();
        Reducer!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_1008 {
    () => {
        deps!();
        impl < 'f , T , C > UnindexedConsumer < Option < T > > for WhileSomeConsumer < 'f , C > where C : UnindexedConsumer < T > , T : Send , { fn split_off_left (& self) -> Self { WhileSomeConsumer { base : self . base . split_off_left () , .. * self } } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_1008!();