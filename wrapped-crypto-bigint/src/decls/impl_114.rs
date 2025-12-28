macro_rules! deps {
    () => {
        Int!();
        One!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < const LIMBS : usize > One for Int < LIMBS > { # [inline (always)] fn one () -> Self { Self :: ONE } }
    };
}

impl_114!()