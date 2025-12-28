macro_rules! NGHTTP2_HCAT_HEADERS {
    () => {
        # [doc = " The HEADERS frame which does not apply for the above categories,"] # [doc = " which is analogous to HEADERS in SPDY.  If non-final response"] # [doc = " (e.g., status 1xx) is used, final response HEADERS frame will be"] # [doc = " categorized here."] pub const NGHTTP2_HCAT_HEADERS : nghttp2_headers_category = 3 ;
    };
}

NGHTTP2_HCAT_HEADERS!()