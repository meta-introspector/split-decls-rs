// Generated macro for other_171 (other)
macro_rules! Depcrateother_171 {
() => {
// Module: crate
// Provides: {"other_171"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when chunk of extension frame"] # [doc = " payload is received."] pub fn nghttp2_session_callbacks_set_on_extension_chunk_recv_callback (cbs : * mut nghttp2_session_callbacks , on_extension_chunk_recv_callback : nghttp2_on_extension_chunk_recv_callback ,) ; }
};
}
