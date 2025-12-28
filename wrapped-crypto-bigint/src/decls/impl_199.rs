macro_rules! deps {
    () => {
        NonZero!();
        Limb!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl From < NonZeroU16 > for NonZero < Limb > { fn from (integer : NonZeroU16) -> Self { Self :: from_u16 (integer) } }
    };
}

impl_199!()