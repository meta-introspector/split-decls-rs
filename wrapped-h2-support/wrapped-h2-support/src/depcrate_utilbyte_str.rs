// Generated macro for byte_str (function)
macro_rules! Depcrate_utilbyte_str {
() => {
// Module: crate::util
// Provides: {"byte_str"}
// Dependencies: {}
pub fn byte_str (s : & str) -> h2 :: frame :: BytesStr { h2 :: frame :: BytesStr :: try_from (Bytes :: copy_from_slice (s . as_bytes ())) . unwrap () }
};
}
