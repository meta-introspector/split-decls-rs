macro_rules! deps {
    () => {
        BidiClass!();
        PropertyCodePointMap!();
    };
}

macro_rules! macro_347 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Data marker for the 'BidiClass' Unicode property"] PropertyEnumBidiClassV1 , PropertyCodePointMap <'static , crate :: props :: BidiClass >, is_singleton = true ,) ;
    };
}

macro_347!()