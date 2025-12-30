// Generated macro for quiche_path_event_free (function)
macro_rules! Depcrate_ffiquiche_path_event_free {
() => {
// Module: crate::ffi
// Provides: {"quiche_path_event_free"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_path_event_free (ev : * mut PathEvent) { drop (unsafe { Box :: from_raw (ev) }) ; }
};
}
