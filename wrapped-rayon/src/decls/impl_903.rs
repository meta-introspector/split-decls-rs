macro_rules! deps {
    () => {
        Reducer!();
        UnindexedConsumer!();
        TryFoldConsumer!();
    };
}

macro_rules! impl_903 {
    () => {
        deps!();
        impl < 'r , U , T , C , ID , F > UnindexedConsumer < T > for TryFoldConsumer < 'r , U , C , ID , F > where C : UnindexedConsumer < U > , F : Fn (U :: Output , T) -> U + Sync , ID : Fn () -> U :: Output + Sync , U : Try + Send , { fn split_off_left (& self) -> Self { TryFoldConsumer { base : self . base . split_off_left () , .. * self } } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_903!();