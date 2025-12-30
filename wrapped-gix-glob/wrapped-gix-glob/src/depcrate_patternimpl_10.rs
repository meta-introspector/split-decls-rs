// Generated macro for impl_10 (impl)
macro_rules! Depcrate_patternimpl_10 {
() => {
// Module: crate::pattern
// Provides: {"impl_10"}
// Dependencies: {}
# [doc = " Instantiation"] impl Pattern { # [doc = " Parse the given `text` as pattern, or return `None` if `text` was empty."] pub fn from_bytes (text : & [u8]) -> Option < Self > { crate :: parse :: pattern (text , true) . map (| (text , mode , first_wildcard_pos) | Pattern { text : text . into () , mode , first_wildcard_pos , }) } # [doc = " Parse the given `text` as pattern without supporting leading `!` or `\\\\!` , or return `None` if `text` was empty."] # [doc = ""] # [doc = " This assures that `text` remains entirely unaltered, but removes built-in support for negation as well."] pub fn from_bytes_without_negation (text : & [u8]) -> Option < Self > { crate :: parse :: pattern (text , false) . map (| (text , mode , first_wildcard_pos) | Pattern { text : text . into () , mode , first_wildcard_pos , }) } }
};
}
