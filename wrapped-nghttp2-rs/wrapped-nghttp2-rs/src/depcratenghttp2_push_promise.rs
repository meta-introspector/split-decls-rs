// Generated macro for nghttp2_push_promise (struct)
macro_rules! Depcratenghttp2_push_promise {
() => {
// Module: crate
// Provides: {"nghttp2_push_promise"}
// Dependencies: {}
# [doc = " @struct"] # [doc = ""] # [doc = " The PUSH_PROMISE frame.  It has the following members:"] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_push_promise { # [doc = " The frame header."] pub hd : nghttp2_frame_hd , # [doc = " The length of the padding in this frame.  This includes PAD_HIGH"] # [doc = " and PAD_LOW."] pub padlen : usize , # [doc = " The name/value pairs."] pub nva : * mut nghttp2_nv , # [doc = " The number of name/value pairs in |nva|."] pub nvlen : usize , # [doc = " The promised stream ID"] pub promised_stream_id : i32 , # [doc = " Reserved bit.  Currently this is always set to 0 and application"] # [doc = " should not expect something useful in here."] pub reserved : u8 , }
};
}
