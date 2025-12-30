// Generated macro for other_169 (other)
macro_rules! Depcrateother_169 {
() => {
// Module: crate
// Provides: {"other_169"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when the library asks the"] # [doc = " application to pack extension frame payload in wire format."] pub fn nghttp2_session_callbacks_set_pack_extension_callback (cbs : * mut nghttp2_session_callbacks , pack_extension_callback : nghttp2_pack_extension_callback ,) ; }
};
}
