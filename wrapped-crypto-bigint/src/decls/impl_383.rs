macro_rules! deps {
    () => {
        Uint!();
    };
}

macro_rules! impl_383 {
    () => {
        deps!();
        impl < const LIMBS : usize > ConstOne for Uint < LIMBS > { const ONE : Self = Self :: ONE ; }
    };
}

impl_383!()