macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_326 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryNfkcInertV1`"] PropertyBinaryNfkcInertV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_326!();