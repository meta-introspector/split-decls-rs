macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_288 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryChangesWhenCasemappedV1`"] PropertyBinaryChangesWhenCasemappedV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_288!();