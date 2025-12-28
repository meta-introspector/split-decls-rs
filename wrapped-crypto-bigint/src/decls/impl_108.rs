macro_rules! deps {
    () => {
        Int!();
        FixedInteger!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < const LIMBS : usize > FixedInteger for Int < LIMBS > { const LIMBS : usize = LIMBS ; }
    };
}

impl_108!()