// Generated macro for quiche_config_set_initial_congestion_window_packets (function)
macro_rules! Depcrate_ffiquiche_config_set_initial_congestion_window_packets {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_set_initial_congestion_window_packets"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_set_initial_congestion_window_packets (config : & mut Config , packets : size_t ,) { config . set_initial_congestion_window_packets (packets) ; }
};
}
