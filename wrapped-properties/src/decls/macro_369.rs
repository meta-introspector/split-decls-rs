macro_rules! deps {
    () => {
        PropertyCodePointMap!();
    };
}

macro_rules! macro_369 {
    () => {
        deps!();
        icu_provider :: data_struct ! (< T : TrieValue > PropertyCodePointMap <'_ , T >, # [cfg (feature = "datagen")]) ;
    };
}

macro_369!();