macro_rules! deps {
    () => {
        Limb!();
        NonZero!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        # [cfg (target_pointer_width = "64")] impl From < NonZeroU64 > for NonZero < Limb > { fn from (integer : NonZeroU64) -> Self { Self :: from_u64 (integer) } }
    };
}

impl_201!();