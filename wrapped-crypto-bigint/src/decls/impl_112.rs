macro_rules! deps {
    () => {
        Int!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < const LIMBS : usize > ConstOne for Int < LIMBS > { const ONE : Self = Self :: ONE ; }
    };
}

impl_112!()