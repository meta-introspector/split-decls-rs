macro_rules! nghttp2_vec {
    () => {
        # [doc = " @struct"] # [doc = ""] # [doc = " The object representing single contiguous buffer."] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_vec { # [doc = " The pointer to the buffer."] pub base : * mut u8 , # [doc = " The length of the buffer."] pub len : usize , }
    };
}

nghttp2_vec!()