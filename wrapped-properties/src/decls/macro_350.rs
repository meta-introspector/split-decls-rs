macro_rules! deps {
    () => {
        PropertyCodePointMap!();
    };
}

macro_rules! macro_350 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Data marker for the 'GeneralCategory' Unicode property"] PropertyEnumGeneralCategoryV1 , PropertyCodePointMap <'static , crate :: props :: GeneralCategory >, is_singleton = true ,) ;
    };
}

macro_350!();