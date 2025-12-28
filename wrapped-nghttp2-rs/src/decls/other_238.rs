macro_rules! other_238 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Returns string representation of HTTP/2 error code |error_code|"] # [doc = " (e.g., ``PROTOCOL_ERROR`` is returned if ``error_code =="] # [doc = " NGHTTP2_PROTOCOL_ERROR``).  If string representation is unknown for"] # [doc = " given |error_code|, this function returns string ``unknown``."] pub fn nghttp2_http2_strerror (error_code : u32) -> * const :: std :: os :: raw :: c_char ; }
    };
}

other_238!()