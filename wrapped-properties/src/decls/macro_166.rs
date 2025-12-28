macro_rules! macro_166 {
    () => {
        make_binary_property ! { name : "Changes_When_Uppercased" ; short_name : "CWU" ; ident : ChangesWhenUppercased ; data_marker : crate :: provider :: PropertyBinaryChangesWhenUppercasedV1 ; singleton : SINGLETON_PROPERTY_BINARY_CHANGES_WHEN_UPPERCASED_V1 ; # [doc = " Characters whose normalized forms are not stable under a `toUppercase` mapping."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::ChangesWhenUppercased;"] # [doc = ""] # [doc = " let changes_when_uppercased = CodePointSetData::new::<ChangesWhenUppercased>();"] # [doc = ""] # [doc = " assert!(changes_when_uppercased.contains('ւ'));  // U+0582 ARMENIAN SMALL LETTER YIWN"] # [doc = " assert!(!changes_when_uppercased.contains('Ւ'));  // U+0552 ARMENIAN CAPITAL LETTER YIWN"] # [doc = " ```"] }
    };
}

macro_166!()