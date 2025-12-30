// Generated macro for macro_210 (macro)
macro_rules! Depcrate_propsmacro_210 {
() => {
// Module: crate::props
// Provides: {"macro_210"}
// Dependencies: {}
make_binary_property ! { name : "Deprecated" ; short_name : "Dep" ; ident : Deprecated ; data_marker : crate :: provider :: PropertyBinaryDeprecatedV1 ; singleton : SINGLETON_PROPERTY_BINARY_DEPRECATED_V1 ; # [doc = " Deprecated characters."] # [doc = ""] # [doc = " No characters will ever be removed from the standard, but the"] # [doc = " usage of deprecated characters is strongly discouraged."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::Deprecated;"] # [doc = ""] # [doc = " let deprecated = CodePointSetData::new::<Deprecated>();"] # [doc = ""] # [doc = " assert!(deprecated.contains('ឣ'));  // U+17A3 KHMER INDEPENDENT VOWEL QAQ"] # [doc = " assert!(!deprecated.contains('A'));"] # [doc = " ```"] }
};
}
