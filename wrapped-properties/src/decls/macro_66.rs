macro_rules! deps {
    () => {
        HangulSyllableType!();
    };
}

macro_rules! macro_66 {
    () => {
        deps!();
        impl_value_getter ! { impl HangulSyllableType { PropertyNameParseHangulSyllableTypeV1 / SINGLETON_PROPERTY_NAME_PARSE_HANGUL_SYLLABLE_TYPE_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameShortHangulSyllableTypeV1 / SINGLETON_PROPERTY_NAME_SHORT_HANGUL_SYLLABLE_TYPE_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameLongHangulSyllableTypeV1 / SINGLETON_PROPERTY_NAME_LONG_HANGUL_SYLLABLE_TYPE_V1 ; } }
    };
}

macro_66!();