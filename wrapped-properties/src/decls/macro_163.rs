macro_rules! macro_163 {
    () => {
        make_binary_property ! { name : "Changes_When_NFKC_Casefolded" ; short_name : "CWKCF" ; ident : ChangesWhenNfkcCasefolded ; data_marker : crate :: provider :: PropertyBinaryChangesWhenNfkcCasefoldedV1 ; singleton : SINGLETON_PROPERTY_BINARY_CHANGES_WHEN_NFKC_CASEFOLDED_V1 ; # [doc = " Characters which are not identical to their `NFKC_Casefold` mapping."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::ChangesWhenNfkcCasefolded;"] # [doc = ""] # [doc = " let changes_when_nfkc_casefolded = CodePointSetData::new::<ChangesWhenNfkcCasefolded>();"] # [doc = ""] # [doc = " assert!(changes_when_nfkc_casefolded.contains('🄵'));  // U+1F135 SQUARED LATIN CAPITAL LETTER F"] # [doc = " assert!(!changes_when_nfkc_casefolded.contains('f'));"] # [doc = " ```"] }
    };
}

macro_163!()