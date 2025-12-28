macro_rules! NGHTTP2_ERR_STREAM_SHUT_WR {
    () => {
        # [doc = " The transmission is not allowed for this stream (e.g., a frame"] # [doc = " with END_STREAM flag set has already sent)."] pub const NGHTTP2_ERR_STREAM_SHUT_WR : nghttp2_error = - 512 ;
    };
}

NGHTTP2_ERR_STREAM_SHUT_WR!()