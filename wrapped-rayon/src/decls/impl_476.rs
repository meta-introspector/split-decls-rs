macro_rules! deps {
    () => {
        UnindexedConsumer!();
        Reducer!();
        FilterMapConsumer!();
    };
}

macro_rules! impl_476 {
    () => {
        deps!();
        impl < 'p , T , U , C , P > UnindexedConsumer < T > for FilterMapConsumer < 'p , C , P > where C : UnindexedConsumer < U > , P : Fn (T) -> Option < U > + Sync + 'p , { fn split_off_left (& self) -> Self { FilterMapConsumer :: new (self . base . split_off_left () , self . filter_op) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_476!();