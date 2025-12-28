macro_rules! deps {
    () => {
        Reducer!();
        CopiedConsumer!();
        UnindexedConsumer!();
    };
}

macro_rules! impl_388 {
    () => {
        deps!();
        impl < 'a , T , C > UnindexedConsumer < & 'a T > for CopiedConsumer < C > where C : UnindexedConsumer < T > , T : 'a + Copy , { fn split_off_left (& self) -> Self { CopiedConsumer :: new (self . base . split_off_left ()) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
    };
}

impl_388!();