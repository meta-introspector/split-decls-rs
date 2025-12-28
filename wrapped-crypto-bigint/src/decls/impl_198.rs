macro_rules! deps {
    () => {
        NonZero!();
        Limb!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl From < NonZeroU8 > for NonZero < Limb > { fn from (integer : NonZeroU8) -> Self { Self :: from_u8 (integer) } }
    };
}

impl_198!();