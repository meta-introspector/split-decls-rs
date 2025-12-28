macro_rules! deps {
    () => {
        DecompositionData!();
    };
}

macro_rules! macro_39 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Marker for data for canonical decomposition."] NormalizerNfdDataV1 , "normalizer/nfd/data/v1" , DecompositionData <'static >, is_singleton = true) ;
    };
}

macro_39!();