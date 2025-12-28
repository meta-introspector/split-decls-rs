macro_rules! deps {
    () => {
        LineBreak!();
    };
}

macro_rules! macro_68 {
    () => {
        deps!();
        impl_value_getter ! { impl LineBreak { PropertyNameParseLineBreakV1 / SINGLETON_PROPERTY_NAME_PARSE_LINE_BREAK_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameShortLineBreakV1 / SINGLETON_PROPERTY_NAME_SHORT_LINE_BREAK_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameLongLineBreakV1 / SINGLETON_PROPERTY_NAME_LONG_LINE_BREAK_V1 ; } }
    };
}

macro_68!();