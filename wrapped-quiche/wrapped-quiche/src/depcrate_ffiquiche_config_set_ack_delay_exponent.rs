// Generated macro for quiche_config_set_ack_delay_exponent (function)
macro_rules! Depcrate_ffiquiche_config_set_ack_delay_exponent {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_set_ack_delay_exponent"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_set_ack_delay_exponent (config : & mut Config , v : u64 ,) { config . set_ack_delay_exponent (v) ; }
};
}
