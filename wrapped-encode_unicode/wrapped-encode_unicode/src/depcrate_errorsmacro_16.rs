// Generated macro for macro_16 (macro)
macro_rules! Depcrate_errorsmacro_16 {
() => {
// Module: crate::errors
// Provides: {"macro_16"}
// Dependencies: {}
simple ! { # [doc = " Error returned when an `[u16; 2]` doesn't form a valid UTF-16 codepoint."] Utf16ArrayError { # [doc = " The first element is a trailing / low surrogate, which is never valid."] FirstIsTrailingSurrogate => "the first element is a trailing surrogate" , # [doc = " The second element is needed, but is not a trailing surrogate."] SecondIsNotTrailingSurrogate => "the second element is needed but is not a trailing surrogate" , } }
};
}
