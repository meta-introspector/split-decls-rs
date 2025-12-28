macro_rules! deps {
    () => {
        IndicSyllabicCategory!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl IndicSyllabicCategory { # [doc = " Returns an ICU4C `UIndicSyllabicCategory` value."] pub const fn to_icu4c_value (self) -> u8 { self . 0 } # [doc = " Constructor from an ICU4C `UIndicSyllabicCategory` value."] pub const fn from_icu4c_value (value : u8) -> Self { Self (value) } }
    };
}

impl_140!()