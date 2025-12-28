macro_rules! deps {
    () => {
        Uint!();
    };
}

macro_rules! impl_382 {
    () => {
        deps!();
        impl < const LIMBS : usize > ConstZero for Uint < LIMBS > { const ZERO : Self = Self :: ZERO ; }
    };
}

impl_382!()