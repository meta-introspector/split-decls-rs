// Generated macro for NGHTTP2_DATA_FLAG_NO_END_STREAM (const)
macro_rules! DepcrateNGHTTP2_DATA_FLAG_NO_END_STREAM {
() => {
// Module: crate
// Provides: {"NGHTTP2_DATA_FLAG_NO_END_STREAM"}
// Dependencies: {}
# [doc = " Indicates that END_STREAM flag must not be set even if"] # [doc = " NGHTTP2_DATA_FLAG_EOF is set.  Usually this flag is used to send"] # [doc = " trailer fields with `nghttp2_submit_request()` or"] # [doc = " `nghttp2_submit_response()`."] pub const NGHTTP2_DATA_FLAG_NO_END_STREAM : nghttp2_data_flag = 2 ;
};
}
