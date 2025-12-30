// Generated macro for bytes2path (function)
macro_rules! Depcrate_utilbytes2path {
() => {
// Module: crate::util
// Provides: {"bytes2path"}
// Dependencies: {}
# [cfg (windows)] pub fn bytes2path (b : & [u8]) -> & Path { use std :: str ; Path :: new (str :: from_utf8 (b) . unwrap ()) }
};
}
