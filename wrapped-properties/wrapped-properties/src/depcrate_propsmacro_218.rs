// Generated macro for macro_218 (macro)
macro_rules! Depcrate_propsmacro_218 {
() => {
// Module: crate::props
// Provides: {"macro_218"}
// Dependencies: {}
make_binary_property ! { name : "Extender" ; short_name : "Ext" ; ident : Extender ; data_marker : crate :: provider :: PropertyBinaryExtenderV1 ; singleton : SINGLETON_PROPERTY_BINARY_EXTENDER_V1 ; # [doc = " Characters whose principal function is to extend the value of a preceding alphabetic"] # [doc = " character or to extend the shape of adjacent characters."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::Extender;"] # [doc = ""] # [doc = " let extender = CodePointSetData::new::<Extender>();"] # [doc = ""] # [doc = " assert!(extender.contains('ヾ'));  // U+30FE KATAKANA VOICED ITERATION MARK"] # [doc = " assert!(extender.contains('ー'));  // U+30FC KATAKANA-HIRAGANA PROLONGED SOUND MARK"] # [doc = " assert!(!extender.contains('・'));  // U+30FB KATAKANA MIDDLE DOT"] # [doc = " ```"] }
};
}
