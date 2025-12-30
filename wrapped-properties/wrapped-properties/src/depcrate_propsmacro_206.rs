// Generated macro for macro_206 (macro)
macro_rules! Depcrate_propsmacro_206 {
() => {
// Module: crate::props
// Provides: {"macro_206"}
// Dependencies: {}
make_binary_property ! { name : "Changes_When_Lowercased" ; short_name : "CWL" ; ident : ChangesWhenLowercased ; data_marker : crate :: provider :: PropertyBinaryChangesWhenLowercasedV1 ; singleton : SINGLETON_PROPERTY_BINARY_CHANGES_WHEN_LOWERCASED_V1 ; # [doc = " Characters whose normalized forms are not stable under a `toLowercase` mapping."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::ChangesWhenLowercased;"] # [doc = ""] # [doc = " let changes_when_lowercased = CodePointSetData::new::<ChangesWhenLowercased>();"] # [doc = ""] # [doc = " assert!(changes_when_lowercased.contains('Ⴔ'));  // U+10B4 GEORGIAN CAPITAL LETTER PHAR"] # [doc = " assert!(!changes_when_lowercased.contains('ფ'));  // U+10E4 GEORGIAN LETTER PHAR"] # [doc = " ```"] }
};
}
