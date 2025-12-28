macro_rules! deps {
    () => {
        Limb!();
        Int!();
        Integer!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < const LIMBS : usize > Integer for Int < LIMBS > { fn as_limbs (& self) -> & [Limb] { self . 0 . as_limbs () } fn as_mut_limbs (& mut self) -> & mut [Limb] { self . 0 . as_mut_limbs () } fn nlimbs (& self) -> usize { self . 0 . nlimbs () } }
    };
}

impl_109!();