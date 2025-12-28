macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_333 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryQuotationMarkV1`"] PropertyBinaryQuotationMarkV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_333!();