macro_rules! deps {
    () => {
        CanonicalCombiningClass!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl CanonicalCombiningClass { # [doc = " Returns an ICU4C `UCanonicalCombiningClass` value."] pub const fn to_icu4c_value (self) -> u8 { self . 0 } # [doc = " Constructor from an ICU4C `UCanonicalCombiningClass` value."] pub const fn from_icu4c_value (value : u8) -> Self { Self (value) } }
    };
}

impl_132!()