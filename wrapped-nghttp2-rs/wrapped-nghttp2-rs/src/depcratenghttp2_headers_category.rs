// Generated macro for nghttp2_headers_category (type)
macro_rules! Depcratenghttp2_headers_category {
() => {
// Module: crate
// Provides: {"nghttp2_headers_category"}
// Dependencies: {}
# [doc = " @enum"] # [doc = ""] # [doc = " The category of HEADERS, which indicates the role of the frame.  In"] # [doc = " HTTP/2 spec, request, response, push response and other arbitrary"] # [doc = " headers (e.g., trailer fields) are all called just HEADERS.  To"] # [doc = " give the application the role of incoming HEADERS frame, we define"] # [doc = " several categories."] pub type nghttp2_headers_category = u32 ;
};
}
