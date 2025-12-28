macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_366 {
    () => {
        deps!();
        icu_provider :: data_struct ! (PropertyCodePointSet <'_ >, # [cfg (feature = "datagen")]) ;
    };
}

macro_366!()