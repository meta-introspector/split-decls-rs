// Generated macro for macro_211 (macro)
macro_rules! Depcrate_propsmacro_211 {
() => {
// Module: crate::props
// Provides: {"macro_211"}
// Dependencies: {}
make_binary_property ! { name : "Default_Ignorable_Code_Point" ; short_name : "DI" ; ident : DefaultIgnorableCodePoint ; data_marker : crate :: provider :: PropertyBinaryDefaultIgnorableCodePointV1 ; singleton : SINGLETON_PROPERTY_BINARY_DEFAULT_IGNORABLE_CODE_POINT_V1 ; # [doc = " For programmatic determination of default ignorable code points."] # [doc = ""] # [doc = " New characters that"] # [doc = " should be ignored in rendering (unless explicitly supported) will be assigned in these"] # [doc = " ranges, permitting programs to correctly handle the default rendering of such"] # [doc = " characters when not otherwise supported."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::DefaultIgnorableCodePoint;"] # [doc = ""] # [doc = " let default_ignorable_code_point = CodePointSetData::new::<DefaultIgnorableCodePoint>();"] # [doc = ""] # [doc = " assert!(default_ignorable_code_point.contains('\\u{180B}'));  // MONGOLIAN FREE VARIATION SELECTOR ONE"] # [doc = " assert!(!default_ignorable_code_point.contains('E'));"] # [doc = " ```"] }
};
}
