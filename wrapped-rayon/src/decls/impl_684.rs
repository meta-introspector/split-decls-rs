macro_rules! deps {
    () => {
        MapConsumer!();
        Reducer!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_684 {
    () => {
        deps!();
        impl < 'f , T , R , C , F > UnindexedConsumer < T > for MapConsumer < 'f , C , F > where C : UnindexedConsumer < F :: Output > , F : Fn (T) -> R + Sync , R : Send , { fn split_off_left (& self) -> Self { MapConsumer :: new (self . base . split_off_left () , self . map_op) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_684!();