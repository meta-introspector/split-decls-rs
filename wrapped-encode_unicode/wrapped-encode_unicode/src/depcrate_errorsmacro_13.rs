// Generated macro for macro_13 (macro)
macro_rules! Depcrate_errorsmacro_13 {
() => {
// Module: crate::errors
// Provides: {"macro_13"}
// Dependencies: {}
simple ! { # [doc = " Error returned when an `u32` is not a valid unicode codepoint."] CodepointError { # [doc = " It's reserved for UTF-16 surrogate pairs."] Utf16Reserved => "is reserved for UTF-16 surrogate pairs" , # [doc = " It's higher than the highest codepoint (which is 0x10ffff)."] TooHigh => "is higher than the highest codepoint" , } }
};
}
