macro_rules! NGHTTP2_ERR_FATAL {
    () => {
        # [doc = " The errors < :enum:`NGHTTP2_ERR_FATAL` mean that the library is"] # [doc = " under unexpected condition and processing was terminated (e.g.,"] # [doc = " out of memory).  If application receives this error code, it must"] # [doc = " stop using that :type:`nghttp2_session` object and only allowed"] # [doc = " operation for that object is deallocate it using"] # [doc = " `nghttp2_session_del()`."] pub const NGHTTP2_ERR_FATAL : nghttp2_error = - 900 ;
    };
}

NGHTTP2_ERR_FATAL!()