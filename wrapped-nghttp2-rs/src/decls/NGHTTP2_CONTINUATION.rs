macro_rules! NGHTTP2_CONTINUATION {
    () => {
        # [doc = " The CONTINUATION frame.  This frame type won't be passed to any"] # [doc = " callbacks because the library processes this frame type and its"] # [doc = " preceding HEADERS/PUSH_PROMISE as a single frame."] pub const NGHTTP2_CONTINUATION : nghttp2_frame_type = 9 ;
    };
}

NGHTTP2_CONTINUATION!();