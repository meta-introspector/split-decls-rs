macro_rules! deps {
    () => {
        CanonicalCompositions!();
    };
}

macro_rules! macro_52 {
    () => {
        deps!();
        icu_provider :: data_struct ! (CanonicalCompositions <'_ >, # [cfg (feature = "datagen")]) ;
    };
}

macro_52!()