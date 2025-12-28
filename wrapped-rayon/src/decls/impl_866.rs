macro_rules! deps {
    () => {
        UnindexedConsumer!();
        SumConsumer!();
        Reducer!();
    };
}

macro_rules! impl_866 {
    () => {
        deps!();
        impl < S , T > UnindexedConsumer < T > for SumConsumer < S > where S : Send + Sum < T > + Sum , { fn split_off_left (& self) -> Self { SumConsumer :: new () } fn to_reducer (& self) -> Self :: Reducer { SumConsumer :: new () } }
    };
}

impl_866!()