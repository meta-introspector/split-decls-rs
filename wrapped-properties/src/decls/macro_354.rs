macro_rules! deps {
    () => {
        PropertyCodePointMap!();
        IndicSyllabicCategory!();
    };
}

macro_rules! macro_354 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Data marker for the 'IndicSyllabicCategory' Unicode property"] PropertyEnumIndicSyllabicCategoryV1 , PropertyCodePointMap <'static , crate :: props :: IndicSyllabicCategory >, is_singleton = true ,) ;
    };
}

macro_354!();