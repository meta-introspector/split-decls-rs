macro_rules! NGHTTP2_NV_FLAG_NO_INDEX {
    () => {
        # [doc = " Indicates that this name/value pair must not be indexed (\"Literal"] # [doc = " Header Field never Indexed\" representation must be used in HPACK"] # [doc = " encoding).  Other implementation calls this bit as \"sensitive\"."] pub const NGHTTP2_NV_FLAG_NO_INDEX : nghttp2_nv_flag = 1 ;
    };
}

NGHTTP2_NV_FLAG_NO_INDEX!();