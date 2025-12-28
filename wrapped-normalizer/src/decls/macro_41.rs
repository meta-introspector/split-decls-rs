macro_rules! deps {
    () => {
        DecompositionData!();
    };
}

macro_rules! macro_41 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Marker for data for compatibility decomposition."] NormalizerNfkdDataV1 , "normalizer/nfkd/data/v1" , DecompositionData <'static >, is_singleton = true) ;
    };
}

macro_41!();