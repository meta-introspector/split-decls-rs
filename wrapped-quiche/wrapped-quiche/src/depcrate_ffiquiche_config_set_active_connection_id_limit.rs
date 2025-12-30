// Generated macro for quiche_config_set_active_connection_id_limit (function)
macro_rules! Depcrate_ffiquiche_config_set_active_connection_id_limit {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_set_active_connection_id_limit"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_set_active_connection_id_limit (config : & mut Config , v : u64 ,) { config . set_active_connection_id_limit (v) ; }
};
}
