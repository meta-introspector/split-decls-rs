macro_rules! nghttp2_origin_entry {
    () => {
        # [doc = " @struct"] # [doc = ""] # [doc = " The single entry of an origin."] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_origin_entry { # [doc = " The pointer to origin.  No validation is made against this field"] # [doc = " by the library.  This is not necessarily NULL-terminated."] pub origin : * mut u8 , # [doc = " The length of the |origin|."] pub origin_len : usize , }
    };
}

nghttp2_origin_entry!()