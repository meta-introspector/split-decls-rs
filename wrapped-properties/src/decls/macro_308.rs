macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_308 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryGraphV1`"] PropertyBinaryGraphV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_308!();