macro_rules! deps {
    () => {
        NonZero!();
        Limb!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl From < NonZeroU32 > for NonZero < Limb > { fn from (integer : NonZeroU32) -> Self { Self :: from_u32 (integer) } }
    };
}

impl_200!();