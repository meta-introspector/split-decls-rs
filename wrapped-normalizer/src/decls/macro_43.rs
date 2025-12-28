macro_rules! deps {
    () => {
        DecompositionData!();
    };
}

macro_rules! macro_43 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Marker for data for UTS-46 decomposition."] NormalizerUts46DataV1 , "normalizer/uts46/data/v1" , DecompositionData <'static >, is_singleton = true) ;
    };
}

macro_43!();