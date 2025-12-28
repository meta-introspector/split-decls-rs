macro_rules! deps {
    () => {
        Integer!();
        Limb!();
        Uint!();
    };
}

macro_rules! impl_379 {
    () => {
        deps!();
        impl < const LIMBS : usize > Integer for Uint < LIMBS > { fn as_limbs (& self) -> & [Limb] { & self . limbs } fn as_mut_limbs (& mut self) -> & mut [Limb] { & mut self . limbs } fn nlimbs (& self) -> usize { Self :: LIMBS } }
    };
}

impl_379!();