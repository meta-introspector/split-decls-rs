// Generated macro for nghttp2_priority_spec (struct)
macro_rules! Depcratenghttp2_priority_spec {
() => {
// Module: crate
// Provides: {"nghttp2_priority_spec"}
// Dependencies: {}
# [doc = " @struct"] # [doc = ""] # [doc = " The structure to specify stream dependency."] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_priority_spec { # [doc = " The stream ID of the stream to depend on.  Specifying 0 makes"] # [doc = " stream not depend any other stream."] pub stream_id : i32 , # [doc = " The weight of this dependency."] pub weight : i32 , # [doc = " nonzero means exclusive dependency"] pub exclusive : u8 , }
};
}
