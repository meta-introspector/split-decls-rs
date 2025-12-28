macro_rules! deps {
    () => {
        NonRecursiveDecompositionSupplement!();
    };
}

macro_rules! macro_45 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Marker for additional data for non-recusrsive composition."] NormalizerNfdSupplementV1 , "normalizer/nfd/supplement/v1" , NonRecursiveDecompositionSupplement <'static >, is_singleton = true) ;
    };
}

macro_45!()