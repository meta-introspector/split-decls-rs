// Generated macro for macro_236 (macro)
macro_rules! Depcrate_propsmacro_236 {
() => {
// Module: crate::props
// Provides: {"macro_236"}
// Dependencies: {}
make_binary_property ! { name : "Lowercase" ; short_name : "Lower" ; ident : Lowercase ; data_marker : crate :: provider :: PropertyBinaryLowercaseV1 ; singleton : SINGLETON_PROPERTY_BINARY_LOWERCASE_V1 ; # [doc = " Lowercase characters."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::Lowercase;"] # [doc = ""] # [doc = " let lowercase = CodePointSetData::new::<Lowercase>();"] # [doc = ""] # [doc = " assert!(lowercase.contains('a'));"] # [doc = " assert!(!lowercase.contains('A'));"] # [doc = " ```"] }
};
}
