macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_342 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryVariationSelectorV1`"] PropertyBinaryVariationSelectorV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_342!();