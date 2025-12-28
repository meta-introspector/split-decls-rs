macro_rules! deps {
    () => {
        GeneralCategoryGroup!();
    };
}

macro_rules! macro_263 {
    () => {
        deps!();
        impl_value_getter ! { impl GeneralCategoryGroup { PropertyNameParseGeneralCategoryMaskV1 / SINGLETON_PROPERTY_NAME_PARSE_GENERAL_CATEGORY_MASK_V1 ; } }
    };
}

macro_263!()