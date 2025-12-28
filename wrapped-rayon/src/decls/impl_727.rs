macro_rules! deps {
    () => {
        NoopReducer!();
        UnindexedConsumer!();
        NoopConsumer!();
    };
}

macro_rules! impl_727 {
    () => {
        deps!();
        impl < T > UnindexedConsumer < T > for NoopConsumer { fn split_off_left (& self) -> Self { NoopConsumer } fn to_reducer (& self) -> NoopReducer { NoopReducer } }
    };
}

impl_727!()