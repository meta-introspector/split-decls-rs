macro_rules! deps {
    () => {
        Int!();
        Limb!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < const LIMBS : usize > AsMut < [Limb] > for Int < LIMBS > { fn as_mut (& mut self) -> & mut [Limb] { self . as_mut_limbs () } }
    };
}

impl_103!();