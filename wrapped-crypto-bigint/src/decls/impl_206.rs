macro_rules! deps {
    () => {
        Uint!();
        NonZero!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < const LIMBS : usize > From < NonZeroU128 > for NonZero < Uint < LIMBS > > { fn from (integer : NonZeroU128) -> Self { Self :: from_u128 (integer) } }
    };
}

impl_206!();