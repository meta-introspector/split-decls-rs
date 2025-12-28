macro_rules! nghttp2_ext_origin {
    () => {
        # [doc = " @struct"] # [doc = ""] # [doc = " The payload of ORIGIN frame.  ORIGIN frame is a non-critical"] # [doc = " extension to HTTP/2 and defined by `RFC 8336"] # [doc = " <https://tools.ietf.org/html/rfc8336>`_."] # [doc = ""] # [doc = " If this frame is received, and"] # [doc = " `nghttp2_option_set_user_recv_extension_type()` is not set, and"] # [doc = " `nghttp2_option_set_builtin_recv_extension_type()` is set for"] # [doc = " :enum:`NGHTTP2_ORIGIN`, ``nghttp2_extension.payload`` will point to"] # [doc = " this struct."] # [doc = ""] # [doc = " It has the following members:"] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_ext_origin { # [doc = " The number of origins contained in |ov|."] pub nov : usize , # [doc = " The pointer to the array of origins contained in ORIGIN frame."] pub ov : * mut nghttp2_origin_entry , }
    };
}

nghttp2_ext_origin!();