macro_rules! deps {
    () => {
        WordBreak!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl WordBreak { # [doc = " Returns an ICU4C `UWordBreak` value."] pub const fn to_icu4c_value (self) -> u8 { self . 0 } # [doc = " Constructor from an ICU4C `UWordBreak` value."] pub const fn from_icu4c_value (value : u8) -> Self { Self (value) } }
    };
}

impl_124!();