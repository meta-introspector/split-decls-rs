// Generated macro for nghttp2_priority (struct)
macro_rules! Depcratenghttp2_priority {
() => {
// Module: crate
// Provides: {"nghttp2_priority"}
// Dependencies: {}
# [doc = " @struct"] # [doc = ""] # [doc = " The PRIORITY frame.  It has the following members:"] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_priority { # [doc = " The frame header."] pub hd : nghttp2_frame_hd , # [doc = " The priority specification."] pub pri_spec : nghttp2_priority_spec , }
};
}
