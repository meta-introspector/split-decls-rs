// Generated macro for macro_259 (macro)
macro_rules! Depcrate_propsmacro_259 {
() => {
// Module: crate::props
// Provides: {"macro_259"}
// Dependencies: {}
make_binary_property ! { name : "White_Space" ; short_name : "space" ; ident : WhiteSpace ; data_marker : crate :: provider :: PropertyBinaryWhiteSpaceV1 ; singleton : SINGLETON_PROPERTY_BINARY_WHITE_SPACE_V1 ; # [doc = " Spaces, separator characters and other control characters which should be treated by"] # [doc = " programming languages as \"white space\" for the purpose of parsing elements."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::CodePointSetData;"] # [doc = " use icu::properties::props::WhiteSpace;"] # [doc = ""] # [doc = " let white_space = CodePointSetData::new::<WhiteSpace>();"] # [doc = ""] # [doc = " assert!(white_space.contains(' '));"] # [doc = " assert!(white_space.contains('\\u{000A}'));  // NEW LINE"] # [doc = " assert!(white_space.contains('\\u{00A0}'));  // NO-BREAK SPACE"] # [doc = " assert!(!white_space.contains('\\u{200B}'));  // ZERO WIDTH SPACE"] # [doc = " ```"] }
};
}
