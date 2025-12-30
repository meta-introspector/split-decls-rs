// Generated macro for macro_201 (macro)
macro_rules! Depcrate_propsmacro_201 {
() => {
// Module: crate::props
// Provides: {"macro_201"}
// Dependencies: {}
make_binary_property ! { name : "Case_Ignorable" ; short_name : "CI" ; ident : CaseIgnorable ; data_marker : crate :: provider :: PropertyBinaryCaseIgnorableV1 ; singleton : SINGLETON_PROPERTY_BINARY_CASE_IGNORABLE_V1 ; # [doc = " Characters which are ignored for casing purposes."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::CaseIgnorable;"] # [doc = ""] # [doc = " let case_ignorable = CodePointSetData::new::<CaseIgnorable>();"] # [doc = ""] # [doc = " assert!(case_ignorable.contains(':'));"] # [doc = " assert!(!case_ignorable.contains('λ'));  // U+03BB GREEK SMALL LETTER LAMBDA"] # [doc = " ```"] }
};
}
