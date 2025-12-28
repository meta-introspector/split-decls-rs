macro_rules! deps {
    () => {
        NoopReducer!();
        UnindexedConsumer!();
        ForEachConsumer!();
    };
}

macro_rules! impl_585 {
    () => {
        deps!();
        impl < 'f , F , T > UnindexedConsumer < T > for ForEachConsumer < 'f , F > where F : Fn (T) + Sync , { fn split_off_left (& self) -> Self { ForEachConsumer { op : self . op } } fn to_reducer (& self) -> NoopReducer { NoopReducer } }
    };
}

impl_585!()