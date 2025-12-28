macro_rules! deps {
    () => {
        Reducer!();
        PanicFuseConsumer!();
        UnindexedConsumer!();
        PanicFuseReducer!();
    };
}

macro_rules! impl_751 {
    () => {
        deps!();
        impl < 'a , T , C > UnindexedConsumer < T > for PanicFuseConsumer < 'a , C > where C : UnindexedConsumer < T > , { fn split_off_left (& self) -> Self { PanicFuseConsumer { base : self . base . split_off_left () , fuse : self . fuse . clone () , } } fn to_reducer (& self) -> Self :: Reducer { PanicFuseReducer { base : self . base . to_reducer () , _fuse : self . fuse . clone () , } } }
    };
}

impl_751!();