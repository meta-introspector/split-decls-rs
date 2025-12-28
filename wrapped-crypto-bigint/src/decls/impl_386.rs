macro_rules! deps {
    () => {
        Zero!();
        Uint!();
    };
}

macro_rules! impl_386 {
    () => {
        deps!();
        impl < const LIMBS : usize > num_traits :: Zero for Uint < LIMBS > { # [inline (always)] fn zero () -> Self { Self :: ZERO } fn is_zero (& self) -> bool { self . ct_eq (& Self :: ZERO) . into () } }
    };
}

impl_386!();