// Generated macro for macro_197 (macro)
macro_rules! Depcrate_propsmacro_197 {
() => {
// Module: crate::props
// Provides: {"macro_197"}
// Dependencies: {}
make_binary_property ! { name : "Bidi_Control" ; short_name : "Bidi_C" ; ident : BidiControl ; data_marker : crate :: provider :: PropertyBinaryBidiControlV1 ; singleton : SINGLETON_PROPERTY_BINARY_BIDI_CONTROL_V1 ; # [doc = " Format control characters which have specific functions in the Unicode Bidirectional"] # [doc = " Algorithm."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::BidiControl;"] # [doc = ""] # [doc = " let bidi_control = CodePointSetData::new::<BidiControl>();"] # [doc = ""] # [doc = " assert!(bidi_control.contains('\\u{200F}'));  // RIGHT-TO-LEFT MARK"] # [doc = " assert!(!bidi_control.contains('ش'));  // U+0634 ARABIC LETTER SHEEN"] # [doc = " ```"] }
};
}
