macro_rules! deps {
    () => {
        IndicConjunctBreak!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl IndicConjunctBreak { # [doc = " Returns an ICU4C `UIndicConjunctBreak` value."] pub const fn to_icu4c_value (self) -> u8 { self . 0 } # [doc = " Constructor from an ICU4C `UIndicConjunctBreak` value."] pub const fn from_icu4c_value (value : u8) -> Self { Self (value) } }
    };
}

impl_136!();