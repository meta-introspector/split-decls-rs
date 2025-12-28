macro_rules! deps {
    () => {
        PropertyCodePointMap!();
        SentenceBreak!();
    };
}

macro_rules! macro_358 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Data marker for the 'SentenceBreak' Unicode property"] PropertyEnumSentenceBreakV1 , PropertyCodePointMap <'static , crate :: props :: SentenceBreak >, is_singleton = true ,) ;
    };
}

macro_358!();