// Generated macro for macro_209 (macro)
macro_rules! Depcrate_propsmacro_209 {
() => {
// Module: crate::props
// Provides: {"macro_209"}
// Dependencies: {}
make_binary_property ! { name : "Dash" ; short_name : "Dash" ; ident : Dash ; data_marker : crate :: provider :: PropertyBinaryDashV1 ; singleton : SINGLETON_PROPERTY_BINARY_DASH_V1 ; # [doc = " Punctuation characters explicitly called out as dashes in the Unicode Standard, plus"] # [doc = " their compatibility equivalents."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::Dash;"] # [doc = ""] # [doc = " let dash = CodePointSetData::new::<Dash>();"] # [doc = ""] # [doc = " assert!(dash.contains('⸺'));  // U+2E3A TWO-EM DASH"] # [doc = " assert!(dash.contains('-'));  // U+002D"] # [doc = " assert!(!dash.contains('='));  // U+003D"] # [doc = " ```"] }
};
}
