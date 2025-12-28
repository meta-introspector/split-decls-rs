macro_rules! NGHTTP2_ERR_STREAM_ID_NOT_AVAILABLE {
    () => {
        # [doc = " Stream ID has reached the maximum value.  Therefore no stream ID"] # [doc = " is available."] pub const NGHTTP2_ERR_STREAM_ID_NOT_AVAILABLE : nghttp2_error = - 509 ;
    };
}

NGHTTP2_ERR_STREAM_ID_NOT_AVAILABLE!()