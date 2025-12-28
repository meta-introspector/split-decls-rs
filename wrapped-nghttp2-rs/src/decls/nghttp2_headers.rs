macro_rules! nghttp2_headers {
    () => {
        # [doc = " @struct"] # [doc = ""] # [doc = " The HEADERS frame.  It has the following members:"] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_headers { # [doc = " The frame header."] pub hd : nghttp2_frame_hd , # [doc = " The length of the padding in this frame.  This includes PAD_HIGH"] # [doc = " and PAD_LOW."] pub padlen : usize , # [doc = " The priority specification"] pub pri_spec : nghttp2_priority_spec , # [doc = " The name/value pairs."] pub nva : * mut nghttp2_nv , # [doc = " The number of name/value pairs in |nva|."] pub nvlen : usize , # [doc = " The category of this HEADERS frame."] pub cat : nghttp2_headers_category , }
    };
}

nghttp2_headers!();