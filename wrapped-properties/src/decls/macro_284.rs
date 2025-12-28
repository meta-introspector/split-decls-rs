macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_284 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryCasedV1`"] PropertyBinaryCasedV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_284!()