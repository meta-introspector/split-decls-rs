macro_rules! deps {
    () => {
        Limb!();
        NonZero!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl NonZero < Limb > { # [doc = " Creates a new non-zero limb in a const context."] # [doc = " Panics if the value is zero."] # [doc = ""] # [doc = " In future versions of Rust it should be possible to replace this with"] # [doc = " `NonZero::new(…).unwrap()`"] pub const fn new_unwrap (n : Limb) -> Self { if n . is_nonzero () . is_true_vartime () { Self (n) } else { panic ! ("Invalid value: zero") } } # [doc = " Create a [`NonZero<Limb>`] from a [`NonZeroU8`] (const-friendly)"] pub const fn from_u8 (n : NonZeroU8) -> Self { Self (Limb :: from_u8 (n . get ())) } # [doc = " Create a [`NonZero<Limb>`] from a [`NonZeroU16`] (const-friendly)"] pub const fn from_u16 (n : NonZeroU16) -> Self { Self (Limb :: from_u16 (n . get ())) } # [doc = " Create a [`NonZero<Limb>`] from a [`NonZeroU32`] (const-friendly)"] pub const fn from_u32 (n : NonZeroU32) -> Self { Self (Limb :: from_u32 (n . get ())) } # [doc = " Create a [`NonZero<Limb>`] from a [`NonZeroU64`] (const-friendly)"] # [cfg (target_pointer_width = "64")] pub const fn from_u64 (n : NonZeroU64) -> Self { Self (Limb :: from_u64 (n . get ())) } }
    };
}

impl_188!();