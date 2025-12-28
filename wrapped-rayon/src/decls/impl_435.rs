macro_rules! deps {
    () => {
        ListReducer!();
        UnindexedConsumer!();
        Reducer!();
        ListConsumer!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        impl < T : Send > UnindexedConsumer < T > for ListConsumer { fn split_off_left (& self) -> Self { Self } fn to_reducer (& self) -> Self :: Reducer { ListReducer } }
    };
}

impl_435!();