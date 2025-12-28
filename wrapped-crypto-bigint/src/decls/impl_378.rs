macro_rules! deps {
    () => {
        Uint!();
        FixedInteger!();
    };
}

macro_rules! impl_378 {
    () => {
        deps!();
        impl < const LIMBS : usize > FixedInteger for Uint < LIMBS > { const LIMBS : usize = LIMBS ; }
    };
}

impl_378!()