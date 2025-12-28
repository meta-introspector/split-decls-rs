macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_312 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryIdCompatMathStartV1`"] PropertyBinaryIdCompatMathStartV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_312!();