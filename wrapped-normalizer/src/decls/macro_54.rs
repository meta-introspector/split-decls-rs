macro_rules! deps {
    () => {
        NonRecursiveDecompositionSupplement!();
    };
}

macro_rules! macro_54 {
    () => {
        deps!();
        icu_provider :: data_struct ! (NonRecursiveDecompositionSupplement <'_ >, # [cfg (feature = "datagen")]) ;
    };
}

macro_54!();