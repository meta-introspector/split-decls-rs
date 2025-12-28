macro_rules! deps {
    () => {
        Script!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl Script { # [doc = " Returns an ICU4C `UScriptCode` value."] pub const fn to_icu4c_value (self) -> u16 { self . 0 } # [doc = " Constructor from an ICU4C `UScriptCode` value."] pub const fn from_icu4c_value (value : u16) -> Self { Self (value) } }
    };
}

impl_104!()