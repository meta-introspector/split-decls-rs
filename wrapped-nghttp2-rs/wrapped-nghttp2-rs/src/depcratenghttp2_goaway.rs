// Generated macro for nghttp2_goaway (struct)
macro_rules! Depcratenghttp2_goaway {
() => {
// Module: crate
// Provides: {"nghttp2_goaway"}
// Dependencies: {}
# [doc = " @struct"] # [doc = ""] # [doc = " The GOAWAY frame.  It has the following members:"] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_goaway { # [doc = " The frame header."] pub hd : nghttp2_frame_hd , # [doc = " The last stream stream ID."] pub last_stream_id : i32 , # [doc = " The error code.  See :type:`nghttp2_error_code`."] pub error_code : u32 , # [doc = " The additional debug data"] pub opaque_data : * mut u8 , # [doc = " The length of |opaque_data| member."] pub opaque_data_len : usize , # [doc = " Reserved bit.  Currently this is always set to 0 and application"] # [doc = " should not expect something useful in here."] pub reserved : u8 , }
};
}
