// Generated macro for NGHTTP2_CONTINUATION (const)
macro_rules! DepcrateNGHTTP2_CONTINUATION {
() => {
// Module: crate
// Provides: {"NGHTTP2_CONTINUATION"}
// Dependencies: {}
# [doc = " The CONTINUATION frame.  This frame type won't be passed to any"] # [doc = " callbacks because the library processes this frame type and its"] # [doc = " preceding HEADERS/PUSH_PROMISE as a single frame."] pub const NGHTTP2_CONTINUATION : nghttp2_frame_type = 9 ;
};
}
