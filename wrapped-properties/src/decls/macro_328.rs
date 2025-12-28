macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_328 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryNoncharacterCodePointV1`"] PropertyBinaryNoncharacterCodePointV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_328!();