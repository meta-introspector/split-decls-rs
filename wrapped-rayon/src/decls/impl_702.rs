macro_rules! deps {
    () => {
        UnindexedConsumer!();
        Reducer!();
        MapWithConsumer!();
    };
}

macro_rules! impl_702 {
    () => {
        deps!();
        impl < 'f , T , U , R , C , F > UnindexedConsumer < T > for MapWithConsumer < 'f , C , U , F > where C : UnindexedConsumer < R > , U : Send + Clone , F : Fn (& mut U , T) -> R + Sync , R : Send , { fn split_off_left (& self) -> Self { MapWithConsumer :: new (self . base . split_off_left () , self . item . clone () , self . map_op) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_702!();