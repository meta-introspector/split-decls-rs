macro_rules! deps {
    () => {
        Reducer!();
        UnindexedConsumer!();
        UpdateConsumer!();
    };
}

macro_rules! impl_978 {
    () => {
        deps!();
        impl < 'f , T , C , F > UnindexedConsumer < T > for UpdateConsumer < 'f , C , F > where C : UnindexedConsumer < T > , F : Fn (& mut T) + Send + Sync , { fn split_off_left (& self) -> Self { UpdateConsumer :: new (self . base . split_off_left () , self . update_op) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_978!()