macro_rules! deps {
    () => {
        PropertyCodePointMap!();
        Script!();
    };
}

macro_rules! macro_357 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Data marker for the 'Script' Unicode property"] PropertyEnumScriptV1 , PropertyCodePointMap <'static , crate :: props :: Script >, is_singleton = true ,) ;
    };
}

macro_357!()