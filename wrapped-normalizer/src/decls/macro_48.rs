macro_rules! deps {
    () => {
        DecompositionData!();
    };
}

macro_rules! macro_48 {
    () => {
        deps!();
        icu_provider :: data_struct ! (DecompositionData <'_ >, # [cfg (feature = "datagen")]) ;
    };
}

macro_48!()