macro_rules! deps {
    () => {
        VerticalOrientation!();
        PropertyCodePointMap!();
    };
}

macro_rules! macro_359 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Data marker for the 'Vertical_Orientation' Unicode property"] PropertyEnumVerticalOrientationV1 , PropertyCodePointMap <'static , crate :: props :: VerticalOrientation >, is_singleton = true ,) ;
    };
}

macro_359!();