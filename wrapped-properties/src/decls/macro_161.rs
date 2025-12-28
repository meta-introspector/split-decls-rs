macro_rules! macro_161 {
    () => {
        make_binary_property ! { name : "Changes_When_Casefolded" ; short_name : "CWCF" ; ident : ChangesWhenCasefolded ; data_marker : crate :: provider :: PropertyBinaryChangesWhenCasefoldedV1 ; singleton : SINGLETON_PROPERTY_BINARY_CHANGES_WHEN_CASEFOLDED_V1 ; # [doc = " Characters whose normalized forms are not stable under case folding."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::ChangesWhenCasefolded;"] # [doc = ""] # [doc = " let changes_when_casefolded = CodePointSetData::new::<ChangesWhenCasefolded>();"] # [doc = ""] # [doc = " assert!(changes_when_casefolded.contains('ß'));  // U+00DF LATIN SMALL LETTER SHARP S"] # [doc = " assert!(!changes_when_casefolded.contains('ᜉ'));  // U+1709 TAGALOG LETTER PA"] # [doc = " ```"] }
    };
}

macro_161!()