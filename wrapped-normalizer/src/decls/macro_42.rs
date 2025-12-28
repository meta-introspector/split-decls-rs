macro_rules! deps {
    () => {
        DecompositionTables!();
    };
}

macro_rules! macro_42 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Marker for additional data for compatibility decomposition."] NormalizerNfkdTablesV1 , "normalizer/nfkd/tables/v1" , DecompositionTables <'static >, is_singleton = true) ;
    };
}

macro_42!()