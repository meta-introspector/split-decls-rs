// Generated macro for other_208 (other)
macro_rules! Depcrateother_208 {
() => {
// Module: crate
// Provides: {"other_208"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Sets the |stream_user_data| to the stream denoted by the"] # [doc = " |stream_id|.  If a stream user data is already set to the stream,"] # [doc = " it is replaced with the |stream_user_data|.  It is valid to specify"] # [doc = " ``NULL`` in the |stream_user_data|, which nullifies the associated"] # [doc = " data pointer."] # [doc = ""] # [doc = " It is valid to set the |stream_user_data| to the stream reserved by"] # [doc = " PUSH_PROMISE frame."] # [doc = ""] # [doc = " This function returns 0 if it succeeds, or one of following"] # [doc = " negative error codes:"] # [doc = ""] # [doc = " :enum:`NGHTTP2_ERR_INVALID_ARGUMENT`"] # [doc = "     The stream does not exist"] pub fn nghttp2_session_set_stream_user_data (session : * mut nghttp2_session , stream_id : i32 , stream_user_data : * mut :: std :: os :: raw :: c_void ,) -> :: std :: os :: raw :: c_int ; }
};
}
