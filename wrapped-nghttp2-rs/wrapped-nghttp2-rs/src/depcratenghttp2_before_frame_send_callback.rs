// Generated macro for nghttp2_before_frame_send_callback (type)
macro_rules! Depcratenghttp2_before_frame_send_callback {
() => {
// Module: crate
// Provides: {"nghttp2_before_frame_send_callback"}
// Dependencies: {}
# [doc = " @functypedef"] # [doc = ""] # [doc = " Callback function invoked just before the non-DATA frame |frame| is"] # [doc = " sent.  The |user_data| pointer is the third argument passed in to"] # [doc = " the call to `nghttp2_session_client_new()` or"] # [doc = " `nghttp2_session_server_new()`."] # [doc = ""] # [doc = " The implementation of this function must return 0 if it succeeds."] # [doc = " It can also return :enum:`NGHTTP2_ERR_CANCEL` to cancel the"] # [doc = " transmission of the given frame."] # [doc = ""] # [doc = " If there is a fatal error while executing this callback, the"] # [doc = " implementation should return :enum:`NGHTTP2_ERR_CALLBACK_FAILURE`,"] # [doc = " which makes `nghttp2_session_send()` and"] # [doc = " `nghttp2_session_mem_send()` functions immediately return"] # [doc = " :enum:`NGHTTP2_ERR_CALLBACK_FAILURE`."] # [doc = ""] # [doc = " If the other value is returned, it is treated as if"] # [doc = " :enum:`NGHTTP2_ERR_CALLBACK_FAILURE` is returned.  But the"] # [doc = " implementation should not rely on this since the library may define"] # [doc = " new return value to extend its capability."] # [doc = ""] # [doc = " To set this callback to :type:`nghttp2_session_callbacks`, use"] # [doc = " `nghttp2_session_callbacks_set_before_frame_send_callback()`."] pub type nghttp2_before_frame_send_callback = :: std :: option :: Option < unsafe extern "C" fn (session : * mut nghttp2_session , frame : * const nghttp2_frame , user_data : * mut :: std :: os :: raw :: c_void ,) -> :: std :: os :: raw :: c_int , > ;
};
}
