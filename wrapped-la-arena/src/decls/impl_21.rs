macro_rules! deps {
    () => {
        RawIdx!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl RawIdx { # [doc = " Constructs a [`RawIdx`] from a u32."] pub const fn from_u32 (u32 : u32) -> Self { RawIdx (u32) } # [doc = " Deconstructs a [`RawIdx`] into the underlying u32."] pub const fn into_u32 (self) -> u32 { self . 0 } }
    };
}

impl_21!();