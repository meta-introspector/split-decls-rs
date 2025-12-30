// Generated macro for quiche_h3_event_headers_has_more_frames (function)
macro_rules! Depcrate_h3_ffiquiche_h3_event_headers_has_more_frames {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_event_headers_has_more_frames"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_event_headers_has_more_frames (ev : & h3 :: Event ,) -> bool { match ev { h3 :: Event :: Headers { more_frames , .. } => * more_frames , _ => unreachable ! () , } }
};
}
