macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_287 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryChangesWhenCasefoldedV1`"] PropertyBinaryChangesWhenCasefoldedV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_287!();