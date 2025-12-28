macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_340 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryUnifiedIdeographV1`"] PropertyBinaryUnifiedIdeographV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_340!()