macro_rules! deps {
    () => {
        VerticalOrientation!();
    };
}

macro_rules! macro_75 {
    () => {
        deps!();
        impl_value_getter ! { impl VerticalOrientation { PropertyNameParseVerticalOrientationV1 / SINGLETON_PROPERTY_NAME_PARSE_VERTICAL_ORIENTATION_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameShortVerticalOrientationV1 / SINGLETON_PROPERTY_NAME_SHORT_VERTICAL_ORIENTATION_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameLongVerticalOrientationV1 / SINGLETON_PROPERTY_NAME_LONG_VERTICAL_ORIENTATION_V1 ; } }
    };
}

macro_75!();