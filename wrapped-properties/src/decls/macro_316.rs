macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_316 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryIdStartV1`"] PropertyBinaryIdStartV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_316!();