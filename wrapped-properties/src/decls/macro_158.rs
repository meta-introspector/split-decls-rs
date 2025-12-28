macro_rules! macro_158 {
    () => {
        make_binary_property ! { name : "Cased" ; short_name : "Cased" ; ident : Cased ; data_marker : crate :: provider :: PropertyBinaryCasedV1 ; singleton : SINGLETON_PROPERTY_BINARY_CASED_V1 ; # [doc = " Uppercase, lowercase, and titlecase characters."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::Cased;"] # [doc = ""] # [doc = " let cased = CodePointSetData::new::<Cased>();"] # [doc = ""] # [doc = " assert!(cased.contains('Ꙡ'));  // U+A660 CYRILLIC CAPITAL LETTER REVERSED TSE"] # [doc = " assert!(!cased.contains('ދ'));  // U+078B THAANA LETTER DHAALU"] # [doc = " ```"] }
    };
}

macro_158!();