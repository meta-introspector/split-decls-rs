macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_305 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryGraphemeBaseV1`"] PropertyBinaryGraphemeBaseV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_305!()