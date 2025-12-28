macro_rules! deps {
    () => {
        Limb!();
        Int!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < const LIMBS : usize > AsRef < [Limb] > for Int < LIMBS > { fn as_ref (& self) -> & [Limb] { self . as_limbs () } }
    };
}

impl_102!();