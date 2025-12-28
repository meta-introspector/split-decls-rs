macro_rules! deps {
    () => {
        Reducer!();
        UnindexedConsumer!();
        MapInitConsumer!();
    };
}

macro_rules! impl_715 {
    () => {
        deps!();
        impl < 'f , T , INIT , U , R , C , F > UnindexedConsumer < T > for MapInitConsumer < 'f , C , INIT , F > where C : UnindexedConsumer < R > , INIT : Fn () -> U + Sync , F : Fn (& mut U , T) -> R + Sync , R : Send , { fn split_off_left (& self) -> Self { MapInitConsumer :: new (self . base . split_off_left () , self . init , self . map_op) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_715!()