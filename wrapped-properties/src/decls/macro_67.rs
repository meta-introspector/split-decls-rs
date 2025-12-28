macro_rules! deps {
    () => {
        EastAsianWidth!();
    };
}

macro_rules! macro_67 {
    () => {
        deps!();
        impl_value_getter ! { impl EastAsianWidth { PropertyNameParseEastAsianWidthV1 / SINGLETON_PROPERTY_NAME_PARSE_EAST_ASIAN_WIDTH_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameShortEastAsianWidthV1 / SINGLETON_PROPERTY_NAME_SHORT_EAST_ASIAN_WIDTH_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameLongEastAsianWidthV1 / SINGLETON_PROPERTY_NAME_LONG_EAST_ASIAN_WIDTH_V1 ; } }
    };
}

macro_67!();