// Generated macro for macro_249 (macro)
macro_rules! Depcrate_propsmacro_249 {
() => {
// Module: crate::props
// Provides: {"macro_249"}
// Dependencies: {}
make_binary_property ! { name : "Radical" ; short_name : "Radical" ; ident : Radical ; data_marker : crate :: provider :: PropertyBinaryRadicalV1 ; singleton : SINGLETON_PROPERTY_BINARY_RADICAL_V1 ; # [doc = " Characters used in the definition of Ideographic Description Sequences."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::Radical;"] # [doc = ""] # [doc = " let radical = CodePointSetData::new::<Radical>();"] # [doc = ""] # [doc = " assert!(radical.contains('⺆'));  // U+2E86 CJK RADICAL BOX"] # [doc = " assert!(!radical.contains('丹'));  // U+F95E CJK COMPATIBILITY IDEOGRAPH-F95E"] # [doc = " ```"] }
};
}
