// Generated macro for macro_20 (macro)
macro_rules! Depcrate_errorsmacro_20 {
() => {
// Module: crate::errors
// Provides: {"macro_20"}
// Dependencies: {}
simple ! { # [doc = " Error returned when [`Utf8Char::from_str()`](../struct.Utf8Char.html#impl-FromStr)"] # [doc = " or [`Utf16Char::from_str()`](../struct.Utf16Char.html#impl-FromStr) fails."] FromStrError { # [doc = " `Utf8Char` and `Utf16Char` cannot store more than a single codepoint."] MultipleCodepoints => "contains more than one codepoint" , # [doc = " `Utf8Char` and `Utf16Char` cannot be empty."] Empty => "is empty" , } }
};
}
