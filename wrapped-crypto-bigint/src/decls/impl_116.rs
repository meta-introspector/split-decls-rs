macro_rules! deps {
    () => {
        One!();
        Int!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < const LIMBS : usize > num_traits :: One for Int < LIMBS > { # [inline (always)] fn one () -> Self { Self :: ONE } fn is_one (& self) -> bool { self . 0 . ct_eq (& Self :: ONE . 0) . into () } }
    };
}

impl_116!()