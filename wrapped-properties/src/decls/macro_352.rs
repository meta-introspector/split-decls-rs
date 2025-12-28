macro_rules! deps {
    () => {
        PropertyCodePointMap!();
        HangulSyllableType!();
    };
}

macro_rules! macro_352 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " Data marker for the 'HangulSyllableType' Unicode property"] PropertyEnumHangulSyllableTypeV1 , PropertyCodePointMap <'static , crate :: props :: HangulSyllableType >, is_singleton = true ,) ;
    };
}

macro_352!();