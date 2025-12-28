macro_rules! deps {
    () => {
        DecompositionTables!();
    };
}

macro_rules! macro_50 {
    () => {
        deps!();
        icu_provider :: data_struct ! (DecompositionTables <'_ >, # [cfg (feature = "datagen")]) ;
    };
}

macro_50!();