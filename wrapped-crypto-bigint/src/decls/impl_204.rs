macro_rules! deps {
    () => {
        Uint!();
        NonZero!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl < const LIMBS : usize > From < NonZeroU32 > for NonZero < Uint < LIMBS > > { fn from (integer : NonZeroU32) -> Self { Self :: from_u32 (integer) } }
    };
}

impl_204!()