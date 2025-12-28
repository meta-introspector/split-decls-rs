macro_rules! deps {
    () => {
        ListReducer!();
        Reducer!();
        UnindexedConsumer!();
        ListStringConsumer!();
    };
}

macro_rules! impl_446 {
    () => {
        deps!();
        impl UnindexedConsumer < char > for ListStringConsumer { fn split_off_left (& self) -> Self { Self } fn to_reducer (& self) -> Self :: Reducer { ListReducer } }
    };
}

impl_446!();