macro_rules! deps {
    () => {
        LineBreak!();
        PropertyCodePointMap!();
    };
}

macro_rules! macro_356 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Data marker for the 'LineBreak' Unicode property"] PropertyEnumLineBreakV1 , PropertyCodePointMap <'static , crate :: props :: LineBreak >, is_singleton = true ,) ;
    };
}

macro_356!();