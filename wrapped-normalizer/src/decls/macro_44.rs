macro_rules! deps {
    () => {
        CanonicalCompositions!();
    };
}

macro_rules! macro_44 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Marker for data for composition."] NormalizerNfcV1 , "normalizer/nfc/v1" , CanonicalCompositions <'static >, is_singleton = true) ;
    };
}

macro_44!()