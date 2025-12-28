macro_rules! NGHTTP2_ERR_DEFERRED {
    () => {
        # [doc = " Used as a return value from"] # [doc = " :func:`nghttp2_data_source_read_callback` to indicate that data"] # [doc = " transfer is postponed.  See"] # [doc = " :func:`nghttp2_data_source_read_callback` for details."] pub const NGHTTP2_ERR_DEFERRED : nghttp2_error = - 508 ;
    };
}

NGHTTP2_ERR_DEFERRED!()