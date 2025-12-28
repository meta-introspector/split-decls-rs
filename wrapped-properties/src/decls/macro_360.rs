macro_rules! deps {
    () => {
        PropertyCodePointMap!();
        WordBreak!();
    };
}

macro_rules! macro_360 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Data marker for the 'WordBreak' Unicode property"] PropertyEnumWordBreakV1 , PropertyCodePointMap <'static , crate :: props :: WordBreak >, is_singleton = true ,) ;
    };
}

macro_360!();