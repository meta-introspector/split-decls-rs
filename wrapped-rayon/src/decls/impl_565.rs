macro_rules! deps {
    () => {
        UnindexedConsumer!();
        FoldWithConsumer!();
        Reducer!();
    };
}

macro_rules! impl_565 {
    () => {
        deps!();
        impl < 'r , U , T , C , F > UnindexedConsumer < T > for FoldWithConsumer < 'r , C , U , F > where C : UnindexedConsumer < U > , F : Fn (U , T) -> U + Sync , U : Send + Clone , { fn split_off_left (& self) -> Self { FoldWithConsumer { base : self . base . split_off_left () , item : self . item . clone () , .. * self } } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_565!()