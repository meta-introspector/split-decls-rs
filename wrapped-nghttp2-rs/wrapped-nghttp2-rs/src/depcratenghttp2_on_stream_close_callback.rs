// Generated macro for nghttp2_on_stream_close_callback (type)
macro_rules! Depcratenghttp2_on_stream_close_callback {
() => {
// Module: crate
// Provides: {"nghttp2_on_stream_close_callback"}
// Dependencies: {}
# [doc = " @functypedef"] # [doc = ""] # [doc = " Callback function invoked when the stream |stream_id| is closed."] # [doc = " The reason of closure is indicated by the |error_code|.  The"] # [doc = " |error_code| is usually one of :enum:`nghttp2_error_code`, but that"] # [doc = " is not guaranteed.  The stream_user_data, which was specified in"] # [doc = " `nghttp2_submit_request()` or `nghttp2_submit_headers()`, is still"] # [doc = " available in this function.  The |user_data| pointer is the third"] # [doc = " argument passed in to the call to `nghttp2_session_client_new()` or"] # [doc = " `nghttp2_session_server_new()`."] # [doc = ""] # [doc = " This function is also called for a stream in reserved state."] # [doc = ""] # [doc = " The implementation of this function must return 0 if it succeeds."] # [doc = " If nonzero is returned, it is treated as fatal error and"] # [doc = " `nghttp2_session_recv()`, `nghttp2_session_mem_recv()`,"] # [doc = " `nghttp2_session_send()`, and `nghttp2_session_mem_send()`"] # [doc = " functions immediately return :enum:`NGHTTP2_ERR_CALLBACK_FAILURE`."] # [doc = ""] # [doc = " To set this callback to :type:`nghttp2_session_callbacks`, use"] # [doc = " `nghttp2_session_callbacks_set_on_stream_close_callback()`."] pub type nghttp2_on_stream_close_callback = :: std :: option :: Option < unsafe extern "C" fn (session : * mut nghttp2_session , stream_id : i32 , error_code : u32 , user_data : * mut :: std :: os :: raw :: c_void ,) -> :: std :: os :: raw :: c_int , > ;
};
}
