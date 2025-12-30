// Generated macro for utf8_latin1_up_to (function)
macro_rules! Depcrate_memutf8_latin1_up_to {
() => {
// Module: crate::mem
// Provides: {"utf8_latin1_up_to"}
// Dependencies: {}
# [doc = " Returns the index of first byte that starts an invalid byte"] # [doc = " sequence or a non-Latin1 byte sequence, or the length of the"] # [doc = " string if there are neither."] pub fn utf8_latin1_up_to (buffer : & [u8]) -> usize { is_utf8_latin1_impl (buffer) . unwrap_or (buffer . len ()) }
};
}
