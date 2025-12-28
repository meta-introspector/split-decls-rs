macro_rules! deps {
    () => {
        TakeAnyWhileConsumer!();
        UnindexedConsumer!();
        Reducer!();
    };
}

macro_rules! impl_892 {
    () => {
        deps!();
        impl < 'p , T , C , P > UnindexedConsumer < T > for TakeAnyWhileConsumer < 'p , C , P > where C : UnindexedConsumer < T > , P : Fn (& T) -> bool + Sync , { fn split_off_left (& self) -> Self { TakeAnyWhileConsumer { base : self . base . split_off_left () , .. * self } } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_892!()