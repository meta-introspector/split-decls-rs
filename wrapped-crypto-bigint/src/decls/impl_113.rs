macro_rules! deps {
    () => {
        Zero!();
        Int!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < const LIMBS : usize > Zero for Int < LIMBS > { # [inline (always)] fn zero () -> Self { Self :: ZERO } }
    };
}

impl_113!()