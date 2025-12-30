// Generated macro for quiche_config_set_max_recv_udp_payload_size (function)
macro_rules! Depcrate_ffiquiche_config_set_max_recv_udp_payload_size {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_set_max_recv_udp_payload_size"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_set_max_recv_udp_payload_size (config : & mut Config , v : size_t ,) { config . set_max_recv_udp_payload_size (v) ; }
};
}
