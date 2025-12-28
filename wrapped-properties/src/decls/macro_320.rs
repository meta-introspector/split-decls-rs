macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_320 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryLogicalOrderExceptionV1`"] PropertyBinaryLogicalOrderExceptionV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_320!()