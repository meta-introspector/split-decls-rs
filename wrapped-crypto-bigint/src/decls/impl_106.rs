macro_rules! deps {
    () => {
        Constants!();
        Int!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < const LIMBS : usize > Constants for Int < LIMBS > { const MAX : Self = Self :: MAX ; }
    };
}

impl_106!()