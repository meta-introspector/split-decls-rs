macro_rules! deps {
    () => {
        Script!();
    };
}

macro_rules! macro_65 {
    () => {
        deps!();
        impl_value_getter ! { impl Script { PropertyNameParseScriptV1 / SINGLETON_PROPERTY_NAME_PARSE_SCRIPT_V1 ; PropertyScriptToIcuScriptMap / PropertyNameShortScriptV1 / SINGLETON_PROPERTY_NAME_SHORT_SCRIPT_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameLongScriptV1 / SINGLETON_PROPERTY_NAME_LONG_SCRIPT_V1 ; } }
    };
}

macro_65!();