macro_rules! nghttp2_ext_altsvc {
    () => {
        # [doc = " @struct"] # [doc = ""] # [doc = " The payload of ALTSVC frame.  ALTSVC frame is a non-critical"] # [doc = " extension to HTTP/2.  If this frame is received, and"] # [doc = " `nghttp2_option_set_user_recv_extension_type()` is not set, and"] # [doc = " `nghttp2_option_set_builtin_recv_extension_type()` is set for"] # [doc = " :enum:`NGHTTP2_ALTSVC`, ``nghttp2_extension.payload`` will point to"] # [doc = " this struct."] # [doc = ""] # [doc = " It has the following members:"] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_ext_altsvc { # [doc = " The pointer to origin which this alternative service is"] # [doc = " associated with.  This is not necessarily NULL-terminated."] pub origin : * mut u8 , # [doc = " The length of the |origin|."] pub origin_len : usize , # [doc = " The pointer to Alt-Svc field value contained in ALTSVC frame."] # [doc = " This is not necessarily NULL-terminated."] pub field_value : * mut u8 , # [doc = " The length of the |field_value|."] pub field_value_len : usize , }
    };
}

nghttp2_ext_altsvc!();