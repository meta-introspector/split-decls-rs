// Generated macro for nghttp2_frame_hd (struct)
macro_rules! Depcratenghttp2_frame_hd {
() => {
// Module: crate
// Provides: {"nghttp2_frame_hd"}
// Dependencies: {}
# [doc = " @struct"] # [doc = " The frame header."] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_frame_hd { # [doc = " The length field of this frame, excluding frame header."] pub length : usize , # [doc = " The stream identifier (aka, stream ID)"] pub stream_id : i32 , # [doc = " The type of this frame.  See `nghttp2_frame_type`."] pub type_ : u8 , # [doc = " The flags."] pub flags : u8 , # [doc = " Reserved bit in frame header.  Currently, this is always set to 0"] # [doc = " and application should not expect something useful in here."] pub reserved : u8 , }
};
}
