macro_rules! deps {
    () => {
        Reducer!();
        UnindexedConsumer!();
        ListVecConsumer!();
        ListReducer!();
    };
}

macro_rules! impl_417 {
    () => {
        deps!();
        impl < T : Send > UnindexedConsumer < T > for ListVecConsumer { fn split_off_left (& self) -> Self { Self } fn to_reducer (& self) -> Self :: Reducer { ListReducer } }
    };
}

impl_417!()