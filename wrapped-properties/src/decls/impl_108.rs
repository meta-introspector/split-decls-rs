macro_rules! deps {
    () => {
        HangulSyllableType!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl HangulSyllableType { # [doc = " Returns an ICU4C `UHangulSyllableType` value."] pub const fn to_icu4c_value (self) -> u8 { self . 0 } # [doc = " Constructor from an ICU4C `UHangulSyllableType` value."] pub const fn from_icu4c_value (value : u8) -> Self { Self (value) } }
    };
}

impl_108!()