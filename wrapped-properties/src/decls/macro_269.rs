macro_rules! deps {
    () => {
        WordBreak!();
    };
}

macro_rules! macro_269 {
    () => {
        deps!();
        impl_value_getter ! { impl WordBreak { PropertyNameParseWordBreakV1 / SINGLETON_PROPERTY_NAME_PARSE_WORD_BREAK_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameShortWordBreakV1 / SINGLETON_PROPERTY_NAME_SHORT_WORD_BREAK_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameLongWordBreakV1 / SINGLETON_PROPERTY_NAME_LONG_WORD_BREAK_V1 ; } }
    };
}

macro_269!();