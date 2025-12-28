macro_rules! deps {
    () => {
        FlattenIterConsumer!();
        Reducer!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_546 {
    () => {
        deps!();
        impl < T , C > UnindexedConsumer < T > for FlattenIterConsumer < C > where C : UnindexedConsumer < T :: Item > , T : IntoIterator , { fn split_off_left (& self) -> Self { FlattenIterConsumer :: new (self . base . split_off_left ()) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_546!();