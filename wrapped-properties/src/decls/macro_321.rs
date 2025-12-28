macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_321 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryLowercaseV1`"] PropertyBinaryLowercaseV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_321!();