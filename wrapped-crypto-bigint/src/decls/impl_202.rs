macro_rules! deps {
    () => {
        Uint!();
        NonZero!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl < const LIMBS : usize > From < NonZeroU8 > for NonZero < Uint < LIMBS > > { fn from (integer : NonZeroU8) -> Self { Self :: from_u8 (integer) } }
    };
}

impl_202!()