macro_rules! deps {
    () => {
        One!();
        Uint!();
    };
}

macro_rules! impl_387 {
    () => {
        deps!();
        impl < const LIMBS : usize > num_traits :: One for Uint < LIMBS > { # [inline (always)] fn one () -> Self { Self :: ONE } fn is_one (& self) -> bool { self . ct_eq (& Self :: ONE) . into () } }
    };
}

impl_387!();