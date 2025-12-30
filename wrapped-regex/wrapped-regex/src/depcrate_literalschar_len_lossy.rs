// Generated macro for char_len_lossy (function)
macro_rules! Depcrate_literalschar_len_lossy {
() => {
// Module: crate::literals
// Provides: {"char_len_lossy"}
// Dependencies: {}
fn char_len_lossy (bytes : & [u8]) -> usize { String :: from_utf8_lossy (bytes) . chars () . count () }
};
}
