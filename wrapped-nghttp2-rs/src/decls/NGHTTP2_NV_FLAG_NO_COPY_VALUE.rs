macro_rules! NGHTTP2_NV_FLAG_NO_COPY_VALUE {
    () => {
        # [doc = " This flag is set solely by application.  If this flag is set, the"] # [doc = " library does not make a copy of header field value.  This could"] # [doc = " improve performance."] pub const NGHTTP2_NV_FLAG_NO_COPY_VALUE : nghttp2_nv_flag = 4 ;
    };
}

NGHTTP2_NV_FLAG_NO_COPY_VALUE!();