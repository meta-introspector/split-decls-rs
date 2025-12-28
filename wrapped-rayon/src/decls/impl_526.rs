macro_rules! deps {
    () => {
        FlatMapIterConsumer!();
        UnindexedConsumer!();
        Reducer!();
    };
}

macro_rules! impl_526 {
    () => {
        deps!();
        impl < 'f , T , U , C , F > UnindexedConsumer < T > for FlatMapIterConsumer < 'f , C , F > where C : UnindexedConsumer < U :: Item > , F : Fn (T) -> U + Sync , U : IntoIterator , { fn split_off_left (& self) -> Self { FlatMapIterConsumer :: new (self . base . split_off_left () , self . map_op) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_526!()