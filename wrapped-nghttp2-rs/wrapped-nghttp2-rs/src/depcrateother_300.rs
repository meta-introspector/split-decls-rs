// Generated macro for other_300 (other)
macro_rules! Depcrateother_300 {
() => {
// Module: crate
// Provides: {"other_300"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns pointer to :type:`nghttp2_stream` object denoted by"] # [doc = " |stream_id|.  If stream was not found, returns NULL."] # [doc = ""] # [doc = " Returns imaginary root stream (see"] # [doc = " `nghttp2_session_get_root_stream()`) if 0 is given in |stream_id|."] # [doc = ""] # [doc = " Unless |stream_id| == 0, the returned pointer is valid until next"] # [doc = " call of `nghttp2_session_send()`, `nghttp2_session_mem_send()`,"] # [doc = " `nghttp2_session_recv()`, and `nghttp2_session_mem_recv()`."] pub fn nghttp2_session_find_stream (session : * mut nghttp2_session , stream_id : i32 ,) -> * mut nghttp2_stream ; }
};
}
