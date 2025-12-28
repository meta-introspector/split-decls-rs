macro_rules! NGHTTP2_NV_FLAG_NO_COPY_NAME {
    () => {
        # [doc = " This flag is set solely by application.  If this flag is set, the"] # [doc = " library does not make a copy of header field name.  This could"] # [doc = " improve performance."] pub const NGHTTP2_NV_FLAG_NO_COPY_NAME : nghttp2_nv_flag = 2 ;
    };
}

NGHTTP2_NV_FLAG_NO_COPY_NAME!()