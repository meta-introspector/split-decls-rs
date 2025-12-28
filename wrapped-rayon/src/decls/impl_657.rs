macro_rules! deps {
    () => {
        UnindexedConsumer!();
        IntersperseConsumer!();
        Reducer!();
    };
}

macro_rules! impl_657 {
    () => {
        deps!();
        impl < C , T > UnindexedConsumer < T > for IntersperseConsumer < C , T > where C : UnindexedConsumer < T > , T : Clone + Send , { fn split_off_left (& self) -> Self { let left = IntersperseConsumer { base : self . base . split_off_left () , item : self . item . clone () , clone_first : self . clone_first . clone () , } ; self . clone_first . set (true) ; left } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_657!()