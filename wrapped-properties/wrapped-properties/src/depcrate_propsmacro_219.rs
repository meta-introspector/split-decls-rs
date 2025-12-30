// Generated macro for macro_219 (macro)
macro_rules! Depcrate_propsmacro_219 {
() => {
// Module: crate::props
// Provides: {"macro_219"}
// Dependencies: {}
make_binary_property ! { name : "Extended_Pictographic" ; short_name : "ExtPict" ; ident : ExtendedPictographic ; data_marker : crate :: provider :: PropertyBinaryExtendedPictographicV1 ; singleton : SINGLETON_PROPERTY_BINARY_EXTENDED_PICTOGRAPHIC_V1 ; # [doc = " Pictographic symbols, as well as reserved ranges in blocks largely associated with"] # [doc = " emoji characters"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::ExtendedPictographic;"] # [doc = ""] # [doc = " let extended_pictographic = CodePointSetData::new::<ExtendedPictographic>();"] # [doc = ""] # [doc = " assert!(extended_pictographic.contains('🥳')); // U+1F973 FACE WITH PARTY HORN AND PARTY HAT"] # [doc = " assert!(!extended_pictographic.contains('🇪'));  // U+1F1EA REGIONAL INDICATOR SYMBOL LETTER E"] # [doc = " ```"] }
};
}
