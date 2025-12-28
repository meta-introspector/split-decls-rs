macro_rules! deps {
    () => {
        BidiClass!();
    };
}

macro_rules! macro_62 {
    () => {
        deps!();
        impl_value_getter ! { impl BidiClass { PropertyNameParseBidiClassV1 / SINGLETON_PROPERTY_NAME_PARSE_BIDI_CLASS_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameShortBidiClassV1 / SINGLETON_PROPERTY_NAME_SHORT_BIDI_CLASS_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameLongBidiClassV1 / SINGLETON_PROPERTY_NAME_LONG_BIDI_CLASS_V1 ; } }
    };
}

macro_62!()