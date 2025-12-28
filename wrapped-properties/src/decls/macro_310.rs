macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_310 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryHyphenV1`"] PropertyBinaryHyphenV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_310!();