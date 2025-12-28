macro_rules! NGHTTP2_ERR_START_STREAM_NOT_ALLOWED {
    () => {
        # [doc = " Starting new stream is not allowed (e.g., GOAWAY has been sent"] # [doc = " and/or received)."] pub const NGHTTP2_ERR_START_STREAM_NOT_ALLOWED : nghttp2_error = - 516 ;
    };
}

NGHTTP2_ERR_START_STREAM_NOT_ALLOWED!()