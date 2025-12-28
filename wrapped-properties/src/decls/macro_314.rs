macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_314 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryIdeographicV1`"] PropertyBinaryIdeographicV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_314!();