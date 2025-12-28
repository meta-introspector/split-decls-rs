macro_rules! deps {
    () => {
        Uint!();
        Limb!();
    };
}

macro_rules! impl_370 {
    () => {
        deps!();
        impl < const LIMBS : usize > AsRef < [Limb] > for Uint < LIMBS > { fn as_ref (& self) -> & [Limb] { self . as_limbs () } }
    };
}

impl_370!()