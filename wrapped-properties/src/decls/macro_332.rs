macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_332 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryPrintV1`"] PropertyBinaryPrintV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_332!();