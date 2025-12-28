macro_rules! deps {
    () => {
        CanonicalCombiningClass!();
        PropertyCodePointMap!();
    };
}

macro_rules! macro_348 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Data marker for the 'CanonicalCombiningClass' Unicode property"] PropertyEnumCanonicalCombiningClassV1 , PropertyCodePointMap <'static , crate :: props :: CanonicalCombiningClass >, is_singleton = true ,) ;
    };
}

macro_348!()