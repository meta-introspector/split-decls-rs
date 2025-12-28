macro_rules! deps {
    () => {
        NonZero!();
        Uint!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl < const LIMBS : usize > From < NonZeroU16 > for NonZero < Uint < LIMBS > > { fn from (integer : NonZeroU16) -> Self { Self :: from_u16 (integer) } }
    };
}

impl_203!();