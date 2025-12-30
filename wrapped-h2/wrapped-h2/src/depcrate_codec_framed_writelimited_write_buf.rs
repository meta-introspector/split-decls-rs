// Generated macro for limited_write_buf (macro)
macro_rules! Depcrate_codec_framed_writelimited_write_buf {
() => {
// Module: crate::codec::framed_write
// Provides: {"limited_write_buf"}
// Dependencies: {}
macro_rules ! limited_write_buf { ($ self : expr) => { { let limit = $ self . max_frame_size () + frame :: HEADER_LEN ; $ self . buf . get_mut () . limit (limit) } } ; }
};
}
