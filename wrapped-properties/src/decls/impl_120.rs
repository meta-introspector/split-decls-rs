macro_rules! deps {
    () => {
        GraphemeClusterBreak!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl GraphemeClusterBreak { # [doc = " Returns an ICU4C `UGraphemeClusterBreak` value."] pub const fn to_icu4c_value (self) -> u8 { self . 0 } # [doc = " Constructor from an ICU4C `UGraphemeClusterBreak` value."] pub const fn from_icu4c_value (value : u8) -> Self { Self (value) } }
    };
}

impl_120!()