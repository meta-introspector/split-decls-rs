macro_rules! deps {
    () => {
        Uint!();
        Zero!();
    };
}

macro_rules! impl_384 {
    () => {
        deps!();
        impl < const LIMBS : usize > Zero for Uint < LIMBS > { # [inline (always)] fn zero () -> Self { Self :: ZERO } }
    };
}

impl_384!()