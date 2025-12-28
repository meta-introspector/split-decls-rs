macro_rules! deps {
    () => {
        Uint!();
        NonZero!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl < const LIMBS : usize > From < NonZeroU64 > for NonZero < Uint < LIMBS > > { fn from (integer : NonZeroU64) -> Self { Self :: from_u64 (integer) } }
    };
}

impl_205!();