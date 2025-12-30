// Generated macro for NGHTTP2_ERR_INVALID_HEADER_BLOCK (const)
macro_rules! DepcrateNGHTTP2_ERR_INVALID_HEADER_BLOCK {
() => {
// Module: crate
// Provides: {"NGHTTP2_ERR_INVALID_HEADER_BLOCK"}
// Dependencies: {}
# [doc = " The received frame contains the invalid header block (e.g., There"] # [doc = " are duplicate header names; or the header names are not encoded"] # [doc = " in US-ASCII character set and not lower cased; or the header name"] # [doc = " is zero-length string; or the header value contains multiple"] # [doc = " in-sequence NUL bytes)."] pub const NGHTTP2_ERR_INVALID_HEADER_BLOCK : nghttp2_error = - 518 ;
};
}
