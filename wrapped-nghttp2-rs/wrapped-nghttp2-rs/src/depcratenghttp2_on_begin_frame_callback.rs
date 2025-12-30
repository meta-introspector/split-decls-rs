// Generated macro for nghttp2_on_begin_frame_callback (type)
macro_rules! Depcratenghttp2_on_begin_frame_callback {
() => {
// Module: crate
// Provides: {"nghttp2_on_begin_frame_callback"}
// Dependencies: {}
# [doc = " @functypedef"] # [doc = ""] # [doc = " Callback function invoked when a frame header is received.  The"] # [doc = " |hd| points to received frame header."] # [doc = ""] # [doc = " Unlike :type:`nghttp2_on_frame_recv_callback`, this callback will"] # [doc = " also be called when frame header of CONTINUATION frame is received."] # [doc = ""] # [doc = " If both :type:`nghttp2_on_begin_frame_callback` and"] # [doc = " :type:`nghttp2_on_begin_headers_callback` are set and HEADERS or"] # [doc = " PUSH_PROMISE is received, :type:`nghttp2_on_begin_frame_callback`"] # [doc = " will be called first."] # [doc = ""] # [doc = " The implementation of this function must return 0 if it succeeds."] # [doc = " If nonzero value is returned, it is treated as fatal error and"] # [doc = " `nghttp2_session_recv()` and `nghttp2_session_mem_recv()` functions"] # [doc = " immediately return :enum:`NGHTTP2_ERR_CALLBACK_FAILURE`."] # [doc = ""] # [doc = " To set this callback to :type:`nghttp2_session_callbacks`, use"] # [doc = " `nghttp2_session_callbacks_set_on_begin_frame_callback()`."] pub type nghttp2_on_begin_frame_callback = :: std :: option :: Option < unsafe extern "C" fn (session : * mut nghttp2_session , hd : * const nghttp2_frame_hd , user_data : * mut :: std :: os :: raw :: c_void ,) -> :: std :: os :: raw :: c_int , > ;
};
}
