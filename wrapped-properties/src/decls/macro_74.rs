macro_rules! deps {
    () => {
        JoiningType!();
    };
}

macro_rules! macro_74 {
    () => {
        deps!();
        impl_value_getter ! { impl JoiningType { PropertyNameParseJoiningTypeV1 / SINGLETON_PROPERTY_NAME_PARSE_JOINING_TYPE_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameShortJoiningTypeV1 / SINGLETON_PROPERTY_NAME_SHORT_JOINING_TYPE_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameLongJoiningTypeV1 / SINGLETON_PROPERTY_NAME_LONG_JOINING_TYPE_V1 ; } }
    };
}

macro_74!();