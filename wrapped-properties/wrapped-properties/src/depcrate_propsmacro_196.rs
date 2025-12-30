// Generated macro for macro_196 (macro)
macro_rules! Depcrate_propsmacro_196 {
() => {
// Module: crate::props
// Provides: {"macro_196"}
// Dependencies: {}
make_binary_property ! { name : "Alphabetic" ; short_name : "Alpha" ; ident : Alphabetic ; data_marker : crate :: provider :: PropertyBinaryAlphabeticV1 ; singleton : SINGLETON_PROPERTY_BINARY_ALPHABETIC_V1 ; # [doc = " Alphabetic characters."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::Alphabetic;"] # [doc = ""] # [doc = " let alphabetic = CodePointSetData::new::<Alphabetic>();"] # [doc = ""] # [doc = " assert!(!alphabetic.contains('3'));"] # [doc = " assert!(!alphabetic.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE"] # [doc = " assert!(alphabetic.contains('A'));"] # [doc = " assert!(alphabetic.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS"] # [doc = " ```"] }
};
}
