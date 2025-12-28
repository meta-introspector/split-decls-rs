macro_rules! nghttp2_on_frame_send_callback {
    () => {
        # [doc = " @functypedef"] # [doc = ""] # [doc = " Callback function invoked after the frame |frame| is sent.  The"] # [doc = " |user_data| pointer is the third argument passed in to the call to"] # [doc = " `nghttp2_session_client_new()` or `nghttp2_session_server_new()`."] # [doc = ""] # [doc = " The implementation of this function must return 0 if it succeeds."] # [doc = " If nonzero is returned, it is treated as fatal error and"] # [doc = " `nghttp2_session_send()` and `nghttp2_session_mem_send()` functions"] # [doc = " immediately return :enum:`NGHTTP2_ERR_CALLBACK_FAILURE`."] # [doc = ""] # [doc = " To set this callback to :type:`nghttp2_session_callbacks`, use"] # [doc = " `nghttp2_session_callbacks_set_on_frame_send_callback()`."] pub type nghttp2_on_frame_send_callback = :: std :: option :: Option < unsafe extern "C" fn (session : * mut nghttp2_session , frame : * const nghttp2_frame , user_data : * mut :: std :: os :: raw :: c_void ,) -> :: std :: os :: raw :: c_int , > ;
    };
}

nghttp2_on_frame_send_callback!();