// Generated macro for macro_207 (macro)
macro_rules! Depcrate_propsmacro_207 {
() => {
// Module: crate::props
// Provides: {"macro_207"}
// Dependencies: {}
make_binary_property ! { name : "Changes_When_Titlecased" ; short_name : "CWT" ; ident : ChangesWhenTitlecased ; data_marker : crate :: provider :: PropertyBinaryChangesWhenTitlecasedV1 ; singleton : SINGLETON_PROPERTY_BINARY_CHANGES_WHEN_TITLECASED_V1 ; # [doc = " Characters whose normalized forms are not stable under a `toTitlecase` mapping."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::ChangesWhenTitlecased;"] # [doc = ""] # [doc = " let changes_when_titlecased = CodePointSetData::new::<ChangesWhenTitlecased>();"] # [doc = ""] # [doc = " assert!(changes_when_titlecased.contains('æ'));  // U+00E6 LATIN SMALL LETTER AE"] # [doc = " assert!(!changes_when_titlecased.contains('Æ'));  // U+00E6 LATIN CAPITAL LETTER AE"] # [doc = " ```"] }
};
}
