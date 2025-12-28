macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_280 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryAsciiHexDigitV1`"] PropertyBinaryAsciiHexDigitV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_280!();