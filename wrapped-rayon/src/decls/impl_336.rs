macro_rules! deps {
    () => {
        Reducer!();
        UnindexedConsumer!();
        ClonedConsumer!();
    };
}

macro_rules! impl_336 {
    () => {
        deps!();
        impl < 'a , T , C > UnindexedConsumer < & 'a T > for ClonedConsumer < C > where C : UnindexedConsumer < T > , T : 'a + Clone , { fn split_off_left (& self) -> Self { ClonedConsumer :: new (self . base . split_off_left ()) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_336!();