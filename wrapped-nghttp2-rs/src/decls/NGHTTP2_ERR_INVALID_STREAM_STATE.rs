macro_rules! NGHTTP2_ERR_INVALID_STREAM_STATE {
    () => {
        # [doc = " The state of the stream is not valid (e.g., DATA cannot be sent"] # [doc = " to the stream if response HEADERS has not been sent)."] pub const NGHTTP2_ERR_INVALID_STREAM_STATE : nghttp2_error = - 514 ;
    };
}

NGHTTP2_ERR_INVALID_STREAM_STATE!()