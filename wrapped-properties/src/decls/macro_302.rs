macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_302 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryExtendedPictographicV1`"] PropertyBinaryExtendedPictographicV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_302!()