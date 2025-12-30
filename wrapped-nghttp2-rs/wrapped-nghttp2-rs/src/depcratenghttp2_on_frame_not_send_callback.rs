// Generated macro for nghttp2_on_frame_not_send_callback (type)
macro_rules! Depcratenghttp2_on_frame_not_send_callback {
() => {
// Module: crate
// Provides: {"nghttp2_on_frame_not_send_callback"}
// Dependencies: {}
# [doc = " @functypedef"] # [doc = ""] # [doc = " Callback function invoked after the non-DATA frame |frame| is not"] # [doc = " sent because of the error.  The error is indicated by the"] # [doc = " |lib_error_code|, which is one of the values defined in"] # [doc = " :type:`nghttp2_error`.  The |user_data| pointer is the third"] # [doc = " argument passed in to the call to `nghttp2_session_client_new()` or"] # [doc = " `nghttp2_session_server_new()`."] # [doc = ""] # [doc = " The implementation of this function must return 0 if it succeeds."] # [doc = " If nonzero is returned, it is treated as fatal error and"] # [doc = " `nghttp2_session_send()` and `nghttp2_session_mem_send()` functions"] # [doc = " immediately return :enum:`NGHTTP2_ERR_CALLBACK_FAILURE`."] # [doc = ""] # [doc = " `nghttp2_session_get_stream_user_data()` can be used to get"] # [doc = " associated data."] # [doc = ""] # [doc = " To set this callback to :type:`nghttp2_session_callbacks`, use"] # [doc = " `nghttp2_session_callbacks_set_on_frame_not_send_callback()`."] pub type nghttp2_on_frame_not_send_callback = :: std :: option :: Option < unsafe extern "C" fn (session : * mut nghttp2_session , frame : * const nghttp2_frame , lib_error_code : :: std :: os :: raw :: c_int , user_data : * mut :: std :: os :: raw :: c_void ,) -> :: std :: os :: raw :: c_int , > ;
};
}
