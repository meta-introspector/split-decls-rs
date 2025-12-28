macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_286 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryCaseSensitiveV1`"] PropertyBinaryCaseSensitiveV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_286!()