// Generated macro for calc_max_continuation_frames (function)
macro_rules! Depcrate_codec_framed_readcalc_max_continuation_frames {
() => {
// Module: crate::codec::framed_read
// Provides: {"calc_max_continuation_frames"}
// Dependencies: {}
fn calc_max_continuation_frames (header_max : usize , frame_max : usize) -> usize { let min_frames_for_list = (header_max / frame_max) . max (1) ; let padding = min_frames_for_list >> 2 ; min_frames_for_list . saturating_add (padding) . max (5) }
};
}
