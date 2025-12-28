macro_rules! NGHTTP2_ERR_STREAM_CLOSING {
    () => {
        # [doc = " RST_STREAM has been added to the outbound queue.  The stream is"] # [doc = " in closing state."] pub const NGHTTP2_ERR_STREAM_CLOSING : nghttp2_error = - 511 ;
    };
}

NGHTTP2_ERR_STREAM_CLOSING!()