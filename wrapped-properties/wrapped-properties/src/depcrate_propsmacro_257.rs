// Generated macro for macro_257 (macro)
macro_rules! Depcrate_propsmacro_257 {
() => {
// Module: crate::props
// Provides: {"macro_257"}
// Dependencies: {}
make_binary_property ! { name : "Uppercase" ; short_name : "Upper" ; ident : Uppercase ; data_marker : crate :: provider :: PropertyBinaryUppercaseV1 ; singleton : SINGLETON_PROPERTY_BINARY_UPPERCASE_V1 ; # [doc = " Uppercase characters."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::Uppercase;"] # [doc = ""] # [doc = " let uppercase = CodePointSetData::new::<Uppercase>();"] # [doc = ""] # [doc = " assert!(uppercase.contains('U'));"] # [doc = " assert!(!uppercase.contains('u'));"] # [doc = " ```"] }
};
}
