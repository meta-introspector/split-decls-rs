macro_rules! deps {
    () => {
        UnindexedConsumer!();
        FilterConsumer!();
        Reducer!();
    };
}

macro_rules! impl_465 {
    () => {
        deps!();
        impl < 'p , T , C , P : 'p > UnindexedConsumer < T > for FilterConsumer < 'p , C , P > where C : UnindexedConsumer < T > , P : Fn (& T) -> bool + Sync , { fn split_off_left (& self) -> Self { FilterConsumer :: new (self . base . split_off_left () , self . filter_op) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_465!()