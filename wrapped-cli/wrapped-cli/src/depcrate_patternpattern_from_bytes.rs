// Generated macro for pattern_from_bytes (function)
macro_rules! Depcrate_patternpattern_from_bytes {
() => {
// Module: crate::pattern
// Provides: {"pattern_from_bytes"}
// Dependencies: {}
# [doc = " Convert arbitrary bytes into a regular expression pattern."] # [doc = ""] # [doc = " This conversion fails if the given pattern is not valid UTF-8, in which"] # [doc = " case, a targeted error with more information about where the invalid UTF-8"] # [doc = " occurs is given. The error also suggests the use of hex escape sequences,"] # [doc = " which are supported by many regex engines."] pub fn pattern_from_bytes (pattern : & [u8] ,) -> Result < & str , InvalidPatternError > { std :: str :: from_utf8 (pattern) . map_err (| err | InvalidPatternError { original : escape (pattern) , valid_up_to : err . valid_up_to () , }) }
};
}
