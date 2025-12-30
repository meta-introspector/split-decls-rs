// Generated macro for quiche_config_set_max_stream_window (function)
macro_rules! Depcrate_ffiquiche_config_set_max_stream_window {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_set_max_stream_window"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_set_max_stream_window (config : & mut Config , v : u64 ,) { config . set_max_stream_window (v) ; }
};
}
