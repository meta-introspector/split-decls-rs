macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_285 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryCaseIgnorableV1`"] PropertyBinaryCaseIgnorableV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_285!();