macro_rules! deps {
    () => {
        Uint!();
        One!();
    };
}

macro_rules! impl_385 {
    () => {
        deps!();
        impl < const LIMBS : usize > One for Uint < LIMBS > { # [inline (always)] fn one () -> Self { Self :: ONE } }
    };
}

impl_385!();