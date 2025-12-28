macro_rules! deps {
    () => {
        Reducer!();
        SkipAnyWhileConsumer!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_841 {
    () => {
        deps!();
        impl < 'p , T , C , P > UnindexedConsumer < T > for SkipAnyWhileConsumer < 'p , C , P > where C : UnindexedConsumer < T > , P : Fn (& T) -> bool + Sync , { fn split_off_left (& self) -> Self { SkipAnyWhileConsumer { base : self . base . split_off_left () , .. * self } } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_841!();