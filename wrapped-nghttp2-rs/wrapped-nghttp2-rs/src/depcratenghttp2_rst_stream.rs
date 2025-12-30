// Generated macro for nghttp2_rst_stream (struct)
macro_rules! Depcratenghttp2_rst_stream {
() => {
// Module: crate
// Provides: {"nghttp2_rst_stream"}
// Dependencies: {}
# [doc = " @struct"] # [doc = ""] # [doc = " The RST_STREAM frame.  It has the following members:"] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_rst_stream { # [doc = " The frame header."] pub hd : nghttp2_frame_hd , # [doc = " The error code.  See :type:`nghttp2_error_code`."] pub error_code : u32 , }
};
}
