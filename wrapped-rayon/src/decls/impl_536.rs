macro_rules! deps {
    () => {
        FlattenConsumer!();
        Reducer!();
        IntoParallelIterator!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_536 {
    () => {
        deps!();
        impl < T , C > UnindexedConsumer < T > for FlattenConsumer < C > where C : UnindexedConsumer < T :: Item > , T : IntoParallelIterator , { fn split_off_left (& self) -> Self { FlattenConsumer :: new (self . base . split_off_left ()) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_536!();