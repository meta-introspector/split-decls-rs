macro_rules! deps {
    () => {
        VerticalOrientation!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl VerticalOrientation { # [doc = " Returns an ICU4C `UVerticalOrientation` value."] pub const fn to_icu4c_value (self) -> u8 { self . 0 } # [doc = " Constructor from an ICU4C `UVerticalOrientation` value."] pub const fn from_icu4c_value (value : u8) -> Self { Self (value) } }
    };
}

impl_148!();