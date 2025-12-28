macro_rules! deps {
    () => {
        LineBreak!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl LineBreak { # [doc = " Returns an ICU4C `ULineBreak` value."] pub const fn to_icu4c_value (self) -> u8 { self . 0 } # [doc = " Constructor from an ICU4C `ULineBreak` value."] pub const fn from_icu4c_value (value : u8) -> Self { Self (value) } }
    };
}

impl_116!()