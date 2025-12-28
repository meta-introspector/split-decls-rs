macro_rules! deps {
    () => {
        CanonicalCombiningClass!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        # [cfg (not (feature = "icu_properties"))] impl CanonicalCombiningClass { const fn from_icu4c_value (v : u8) -> Self { Self (v) } const fn to_icu4c_value (self) -> u8 { self . 0 } }
    };
}

impl_68!();