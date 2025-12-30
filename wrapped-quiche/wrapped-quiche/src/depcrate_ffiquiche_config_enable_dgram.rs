// Generated macro for quiche_config_enable_dgram (function)
macro_rules! Depcrate_ffiquiche_config_enable_dgram {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_enable_dgram"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_enable_dgram (config : & mut Config , enabled : bool , recv_queue_len : size_t , send_queue_len : size_t ,) { config . enable_dgram (enabled , recv_queue_len , send_queue_len) ; }
};
}
