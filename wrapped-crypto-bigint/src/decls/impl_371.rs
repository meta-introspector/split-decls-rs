macro_rules! deps {
    () => {
        Limb!();
        Uint!();
    };
}

macro_rules! impl_371 {
    () => {
        deps!();
        impl < const LIMBS : usize > AsMut < [Limb] > for Uint < LIMBS > { fn as_mut (& mut self) -> & mut [Limb] { self . as_mut_limbs () } }
    };
}

impl_371!()