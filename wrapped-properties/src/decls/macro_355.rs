macro_rules! deps {
    () => {
        JoiningType!();
        PropertyCodePointMap!();
    };
}

macro_rules! macro_355 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Data marker for the 'JoiningType' Unicode property"] PropertyEnumJoiningTypeV1 , PropertyCodePointMap <'static , crate :: props :: JoiningType >, is_singleton = true ,) ;
    };
}

macro_355!();