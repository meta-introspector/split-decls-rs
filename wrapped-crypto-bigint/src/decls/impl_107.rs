macro_rules! deps {
    () => {
        Int!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < const LIMBS : usize > Default for Int < LIMBS > { fn default () -> Self { Self :: ZERO } }
    };
}

impl_107!()