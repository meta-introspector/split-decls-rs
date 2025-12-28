macro_rules! deps {
    () => {
        DecompositionTables!();
    };
}

macro_rules! macro_40 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Marker for additional data for canonical decomposition."] NormalizerNfdTablesV1 , "normalizer/nfd/tables/v1" , DecompositionTables <'static >, is_singleton = true) ;
    };
}

macro_40!()