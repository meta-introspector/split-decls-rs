// Generated macro for is_utf16_latin1 (function)
macro_rules! Depcrate_memis_utf16_latin1 {
() => {
// Module: crate::mem
// Provides: {"is_utf16_latin1"}
// Dependencies: {}
# [doc = " Checks whether the buffer represents only code point less than or equal"] # [doc = " to U+00FF."] # [doc = ""] # [doc = " May read the entire buffer even if it isn't all-Latin1. (I.e. the function"] # [doc = " is not guaranteed to fail fast.)"] pub fn is_utf16_latin1 (buffer : & [u16]) -> bool { is_utf16_latin1_impl (buffer) }
};
}
