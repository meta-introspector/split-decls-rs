macro_rules! deps {
    () => {
        IndicSyllabicCategory!();
    };
}

macro_rules! macro_272 {
    () => {
        deps!();
        impl_value_getter ! { impl IndicSyllabicCategory { PropertyNameParseIndicSyllabicCategoryV1 / SINGLETON_PROPERTY_NAME_PARSE_INDIC_SYLLABIC_CATEGORY_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameShortIndicSyllabicCategoryV1 / SINGLETON_PROPERTY_NAME_SHORT_INDIC_SYLLABIC_CATEGORY_V1 ; PropertyEnumToValueNameLinearMap / PropertyNameLongIndicSyllabicCategoryV1 / SINGLETON_PROPERTY_NAME_LONG_INDIC_SYLLABIC_CATEGORY_V1 ; } }
    };
}

macro_272!()