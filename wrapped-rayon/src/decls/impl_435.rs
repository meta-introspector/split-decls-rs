macro_rules! deps {
    () => {
        UnindexedConsumer!();
        ListConsumer!();
        Reducer!();
        ListReducer!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        impl < T : Send > UnindexedConsumer < T > for ListConsumer { fn split_off_left (& self) -> Self { Self } fn to_reducer (& self) -> Self :: Reducer { ListReducer } }
    };
}

impl_435!()