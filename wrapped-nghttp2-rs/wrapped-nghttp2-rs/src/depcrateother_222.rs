// Generated macro for other_222 (other)
macro_rules! Depcrateother_222 {
() => {
// Module: crate
// Provides: {"other_222"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns the current dynamic table size of HPACK deflater including"] # [doc = " the overhead 32 bytes per entry described in RFC 7541."] pub fn nghttp2_session_get_hd_deflate_dynamic_table_size (session : * mut nghttp2_session ,) -> usize ; }
};
}
