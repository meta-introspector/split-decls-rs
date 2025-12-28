macro_rules! deps {
    () => {
        Int!();
        Zero!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < const LIMBS : usize > num_traits :: Zero for Int < LIMBS > { # [inline (always)] fn zero () -> Self { Self :: ZERO } fn is_zero (& self) -> bool { self . 0 . ct_eq (& Self :: ZERO . 0) . into () } }
    };
}

impl_115!()