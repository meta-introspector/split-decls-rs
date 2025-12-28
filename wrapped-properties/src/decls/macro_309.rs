macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_309 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryHexDigitV1`"] PropertyBinaryHexDigitV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_309!();