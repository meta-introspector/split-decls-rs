macro_rules! deps {
    () => {
        Int!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < const LIMBS : usize > ConstZero for Int < LIMBS > { const ZERO : Self = Self :: ZERO ; }
    };
}

impl_111!()