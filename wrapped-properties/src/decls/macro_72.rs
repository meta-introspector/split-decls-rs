macro_rules! deps {
    () => {
        CanonicalCombiningClass!();
    };
}

macro_rules! macro_72 {
    () => {
        deps!();
        impl_value_getter ! { impl CanonicalCombiningClass { PropertyNameParseCanonicalCombiningClassV1 / SINGLETON_PROPERTY_NAME_PARSE_CANONICAL_COMBINING_CLASS_V1 ; # [cfg (feature = "alloc")] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] PropertyEnumToValueNameSparseMap / PropertyNameShortCanonicalCombiningClassV1 / SINGLETON_PROPERTY_NAME_SHORT_CANONICAL_COMBINING_CLASS_V1 ; PropertyEnumToValueNameSparseMap / PropertyNameLongCanonicalCombiningClassV1 / SINGLETON_PROPERTY_NAME_LONG_CANONICAL_COMBINING_CLASS_V1 ; } }
    };
}

macro_72!();