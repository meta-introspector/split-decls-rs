// Generated macro for other_170 (other)
macro_rules! Depcrateother_170 {
() => {
// Module: crate
// Provides: {"other_170"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when the library asks the"] # [doc = " application to unpack extension frame payload from wire format."] pub fn nghttp2_session_callbacks_set_unpack_extension_callback (cbs : * mut nghttp2_session_callbacks , unpack_extension_callback : nghttp2_unpack_extension_callback ,) ; }
};
}
