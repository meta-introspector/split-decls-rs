// Generated macro for quiche_config_set_max_ack_delay (function)
macro_rules! Depcrate_ffiquiche_config_set_max_ack_delay {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_set_max_ack_delay"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_set_max_ack_delay (config : & mut Config , v : u64) { config . set_max_ack_delay (v) ; }
};
}
