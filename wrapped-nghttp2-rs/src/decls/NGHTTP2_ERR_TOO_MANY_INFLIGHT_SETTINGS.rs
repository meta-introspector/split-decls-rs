macro_rules! NGHTTP2_ERR_TOO_MANY_INFLIGHT_SETTINGS {
    () => {
        # [doc = " There are too many in-flight SETTING frame and no more"] # [doc = " transmission of SETTINGS is allowed."] pub const NGHTTP2_ERR_TOO_MANY_INFLIGHT_SETTINGS : nghttp2_error = - 527 ;
    };
}

NGHTTP2_ERR_TOO_MANY_INFLIGHT_SETTINGS!();