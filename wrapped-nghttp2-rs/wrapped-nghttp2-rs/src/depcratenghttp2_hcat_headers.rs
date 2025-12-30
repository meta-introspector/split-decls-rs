// Generated macro for NGHTTP2_HCAT_HEADERS (const)
macro_rules! DepcrateNGHTTP2_HCAT_HEADERS {
() => {
// Module: crate
// Provides: {"NGHTTP2_HCAT_HEADERS"}
// Dependencies: {}
# [doc = " The HEADERS frame which does not apply for the above categories,"] # [doc = " which is analogous to HEADERS in SPDY.  If non-final response"] # [doc = " (e.g., status 1xx) is used, final response HEADERS frame will be"] # [doc = " categorized here."] pub const NGHTTP2_HCAT_HEADERS : nghttp2_headers_category = 3 ;
};
}
