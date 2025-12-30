// Generated macro for pattern_from_os (function)
macro_rules! Depcrate_patternpattern_from_os {
() => {
// Module: crate::pattern
// Provides: {"pattern_from_os"}
// Dependencies: {}
# [doc = " Convert an OS string into a regular expression pattern."] # [doc = ""] # [doc = " This conversion fails if the given pattern is not valid UTF-8, in which"] # [doc = " case, a targeted error with more information about where the invalid UTF-8"] # [doc = " occurs is given. The error also suggests the use of hex escape sequences,"] # [doc = " which are supported by many regex engines."] pub fn pattern_from_os (pattern : & OsStr) -> Result < & str , InvalidPatternError > { pattern . to_str () . ok_or_else (| | { let valid_up_to = pattern . to_string_lossy () . find ('\u{FFFD}') . expect ("a Unicode replacement codepoint for invalid UTF-8") ; InvalidPatternError { original : escape_os (pattern) , valid_up_to } }) }
};
}
