// Generated macro for other_249 (other)
macro_rules! Depcrateother_249 {
() => {
// Module: crate
// Provides: {"other_249"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Submits RST_STREAM frame to cancel/reject the stream |stream_id|"] # [doc = " with the error code |error_code|."] # [doc = ""] # [doc = " The pre-defined error code is one of :enum:`nghttp2_error_code`."] # [doc = ""] # [doc = " The |flags| is currently ignored and should be"] # [doc = " :enum:`NGHTTP2_FLAG_NONE`."] # [doc = ""] # [doc = " This function returns 0 if it succeeds, or one of the following"] # [doc = " negative error codes:"] # [doc = ""] # [doc = " :enum:`NGHTTP2_ERR_NOMEM`"] # [doc = "     Out of memory."] # [doc = " :enum:`NGHTTP2_ERR_INVALID_ARGUMENT`"] # [doc = "     The |stream_id| is 0."] pub fn nghttp2_submit_rst_stream (session : * mut nghttp2_session , flags : u8 , stream_id : i32 , error_code : u32 ,) -> :: std :: os :: raw :: c_int ; }
};
}
