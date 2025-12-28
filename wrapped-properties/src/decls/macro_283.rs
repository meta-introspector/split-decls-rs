macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_283 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryBlankV1`"] PropertyBinaryBlankV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_283!()