macro_rules! deps {
    () => {
        PropertyCodePointMap!();
        GraphemeClusterBreak!();
    };
}

macro_rules! macro_351 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Data marker for the 'GraphemeClusterBreak' Unicode property"] PropertyEnumGraphemeClusterBreakV1 , PropertyCodePointMap <'static , crate :: props :: GraphemeClusterBreak >, is_singleton = true ,) ;
    };
}

macro_351!()