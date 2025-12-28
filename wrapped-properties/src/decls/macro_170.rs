macro_rules! macro_170 {
    () => {
        make_binary_property ! { name : "Diacritic" ; short_name : "Dia" ; ident : Diacritic ; data_marker : crate :: provider :: PropertyBinaryDiacriticV1 ; singleton : SINGLETON_PROPERTY_BINARY_DIACRITIC_V1 ; # [doc = " Characters that linguistically modify the meaning of another character to which they apply."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::Diacritic;"] # [doc = ""] # [doc = " let diacritic = CodePointSetData::new::<Diacritic>();"] # [doc = ""] # [doc = " assert!(diacritic.contains('\\u{05B3}'));  // HEBREW POINT HATAF QAMATS"] # [doc = " assert!(!diacritic.contains('א'));  // U+05D0 HEBREW LETTER ALEF"] # [doc = " ```"] }
    };
}

macro_170!()