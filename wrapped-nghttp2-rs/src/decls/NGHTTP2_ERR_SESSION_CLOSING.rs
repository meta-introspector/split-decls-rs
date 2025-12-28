macro_rules! NGHTTP2_ERR_SESSION_CLOSING {
    () => {
        # [doc = " The current session is closing due to a connection error or"] # [doc = " `nghttp2_session_terminate_session()` is called."] pub const NGHTTP2_ERR_SESSION_CLOSING : nghttp2_error = - 530 ;
    };
}

NGHTTP2_ERR_SESSION_CLOSING!();