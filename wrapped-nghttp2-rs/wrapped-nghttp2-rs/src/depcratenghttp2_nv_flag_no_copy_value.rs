// Generated macro for NGHTTP2_NV_FLAG_NO_COPY_VALUE (const)
macro_rules! DepcrateNGHTTP2_NV_FLAG_NO_COPY_VALUE {
() => {
// Module: crate
// Provides: {"NGHTTP2_NV_FLAG_NO_COPY_VALUE"}
// Dependencies: {}
# [doc = " This flag is set solely by application.  If this flag is set, the"] # [doc = " library does not make a copy of header field value.  This could"] # [doc = " improve performance."] pub const NGHTTP2_NV_FLAG_NO_COPY_VALUE : nghttp2_nv_flag = 4 ;
};
}
