macro_rules! deps {
    () => {
        Limb!();
        Uint!();
        Monty!();
        Unsigned!();
    };
}

macro_rules! impl_380 {
    () => {
        deps!();
        impl < const LIMBS : usize > Unsigned for Uint < LIMBS > { type Monty = MontyForm < LIMBS > ; fn from_limb_like (limb : Limb , _other : & Self) -> Self { Self :: from (limb) } }
    };
}

impl_380!();