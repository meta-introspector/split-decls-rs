macro_rules! deps {
    () => {
        IndicConjunctBreak!();
        PropertyCodePointMap!();
    };
}

macro_rules! macro_353 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Data marker for the 'IndicConjunctBreak' Unicode property"] PropertyEnumIndicConjunctBreakV1 , PropertyCodePointMap <'static , crate :: props :: IndicConjunctBreak >, is_singleton = true ,) ;
    };
}

macro_353!()