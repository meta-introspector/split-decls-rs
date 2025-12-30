// Generated macro for quiche_h3_event_free (function)
macro_rules! Depcrate_h3_ffiquiche_h3_event_free {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_event_free"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_event_free (ev : * mut h3 :: Event) { drop (unsafe { Box :: from_raw (ev) }) ; }
};
}
