macro_rules! other_269 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns nonzero if HTTP header field value |value| of length |len|"] # [doc = " is valid according to"] # [doc = " http://tools.ietf.org/html/rfc7230#section-3.2"] pub fn nghttp2_check_header_value (value : * const u8 , len : usize) -> :: std :: os :: raw :: c_int ; }
    };
}

other_269!();