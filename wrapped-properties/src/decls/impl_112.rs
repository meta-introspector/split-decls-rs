macro_rules! deps {
    () => {
        EastAsianWidth!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl EastAsianWidth { # [doc = " Returns an ICU4C `UEastAsianWidth` value."] pub const fn to_icu4c_value (self) -> u8 { self . 0 } # [doc = " Constructor from an ICU4C `UEastAsianWidth` value."] pub const fn from_icu4c_value (value : u8) -> Self { Self (value) } }
    };
}

impl_112!();