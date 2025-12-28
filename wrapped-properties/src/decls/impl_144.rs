macro_rules! deps {
    () => {
        JoiningType!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl JoiningType { # [doc = " Returns an ICU4C `UJoiningType` value."] pub const fn to_icu4c_value (self) -> u8 { self . 0 } # [doc = " Constructor from an ICU4C `UJoiningType` value."] pub const fn from_icu4c_value (value : u8) -> Self { Self (value) } }
    };
}

impl_144!()