macro_rules! deps {
    () => {
        Reducer!();
        IntoParallelIterator!();
        FlatMapConsumer!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_515 {
    () => {
        deps!();
        impl < 'f , T , U , C , F > UnindexedConsumer < T > for FlatMapConsumer < 'f , C , F > where C : UnindexedConsumer < U :: Item > , F : Fn (T) -> U + Sync , U : IntoParallelIterator , { fn split_off_left (& self) -> Self { FlatMapConsumer :: new (self . base . split_off_left () , self . map_op) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_515!()