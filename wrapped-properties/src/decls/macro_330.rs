macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_330 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryPatternWhiteSpaceV1`"] PropertyBinaryPatternWhiteSpaceV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_330!()