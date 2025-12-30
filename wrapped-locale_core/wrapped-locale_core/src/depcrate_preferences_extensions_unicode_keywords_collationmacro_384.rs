// Generated macro for macro_384 (macro)
macro_rules! Depcrate_preferences_extensions_unicode_keywords_collationmacro_384 {
() => {
// Module: crate::preferences::extensions::unicode::keywords::collation
// Provides: {"macro_384"}
// Dependencies: {}
enum_keyword ! (# [doc = " Collation parameter key for numeric handling."] # [doc = ""] # [doc = " If set to on, any sequence of Decimal Digits (General_Category = Nd in the UAX44) is sorted at a primary level with"] # [doc = " its numeric value. For example, \"1\" < \"2\" < \"10\". The computed primary weights are all at the start of the digit"] # [doc = " reordering group."] [Default] CollationNumericOrdering { # [doc = " A sequence of decimal digits is sorted at primary level with its numeric value"] ("true" => True) , # [doc = " No special handling for numeric ordering"] [default] ("false" => False) , } , "kn") ;
};
}
