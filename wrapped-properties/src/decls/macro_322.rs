macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_322 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryMathV1`"] PropertyBinaryMathV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_322!()