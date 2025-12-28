macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_343 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryWhiteSpaceV1`"] PropertyBinaryWhiteSpaceV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_343!();