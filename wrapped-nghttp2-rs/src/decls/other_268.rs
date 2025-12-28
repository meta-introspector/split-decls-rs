macro_rules! other_268 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns nonzero if HTTP header field name |name| of length |len| is"] # [doc = " valid according to http://tools.ietf.org/html/rfc7230#section-3.2"] # [doc = ""] # [doc = " Because this is a header field name in HTTP2, the upper cased alphabet"] # [doc = " is treated as error."] pub fn nghttp2_check_header_name (name : * const u8 , len : usize) -> :: std :: os :: raw :: c_int ; }
    };
}

other_268!()