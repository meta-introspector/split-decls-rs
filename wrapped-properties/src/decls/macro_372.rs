macro_rules! deps {
    () => {
        PropertyUnicodeSet!();
    };
}

macro_rules! macro_372 {
    () => {
        deps!();
        icu_provider :: data_struct ! (PropertyUnicodeSet <'_ >, # [cfg (feature = "datagen")]) ;
    };
}

macro_372!();