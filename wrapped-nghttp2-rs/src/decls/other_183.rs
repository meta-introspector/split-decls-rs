macro_rules! other_183 {
    () => {
        extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " By default, nghttp2 library, if configured as server, requires"] # [doc = " first 24 bytes of client magic byte string (MAGIC).  In most cases,"] # [doc = " this will simplify the implementation of server.  But sometimes"] # [doc = " server may want to detect the application protocol based on first"] # [doc = " few bytes on clear text communication."] # [doc = ""] # [doc = " If this option is used with nonzero |val|, nghttp2 library does not"] # [doc = " handle MAGIC.  It still checks following SETTINGS frame.  This"] # [doc = " means that applications should deal with MAGIC by themselves."] # [doc = ""] # [doc = " If this option is not used or used with zero value, if MAGIC does"] # [doc = " not match :macro:`NGHTTP2_CLIENT_MAGIC`, `nghttp2_session_recv()`"] # [doc = " and `nghttp2_session_mem_recv()` will return error"] # [doc = " :enum:`NGHTTP2_ERR_BAD_CLIENT_MAGIC`, which is fatal error."] pub fn nghttp2_option_set_no_recv_client_magic (option : * mut nghttp2_option , val : :: std :: os :: raw :: c_int ,) ; }
    };
}

other_183!();