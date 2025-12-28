macro_rules! deps {
    () => {
        EastAsianWidth!();
        PropertyCodePointMap!();
    };
}

macro_rules! macro_349 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Data marker for the 'EastAsianWidth' Unicode property"] PropertyEnumEastAsianWidthV1 , PropertyCodePointMap <'static , crate :: props :: EastAsianWidth >, is_singleton = true ,) ;
    };
}

macro_349!()