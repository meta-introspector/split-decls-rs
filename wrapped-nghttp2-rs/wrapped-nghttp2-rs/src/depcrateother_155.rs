// Generated macro for other_155 (other)
macro_rules! Depcrateother_155 {
() => {
// Module: crate
// Provides: {"other_155"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets callback function invoked when a chunk of data in DATA frame"] # [doc = " is received."] pub fn nghttp2_session_callbacks_set_on_data_chunk_recv_callback (cbs : * mut nghttp2_session_callbacks , on_data_chunk_recv_callback : nghttp2_on_data_chunk_recv_callback ,) ; }
};
}
