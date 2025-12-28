macro_rules! deps {
    () => {
        Reducer!();
        UnindexedConsumer!();
        InspectConsumer!();
    };
}

macro_rules! impl_622 {
    () => {
        deps!();
        impl < 'f , T , C , F > UnindexedConsumer < T > for InspectConsumer < 'f , C , F > where C : UnindexedConsumer < T > , F : Fn (& T) + Sync , { fn split_off_left (& self) -> Self { InspectConsumer :: new (self . base . split_off_left () , self . inspect_op) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_622!()