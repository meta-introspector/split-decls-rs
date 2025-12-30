// Generated macro for quiche_config_set_initial_max_data (function)
macro_rules! Depcrate_ffiquiche_config_set_initial_max_data {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_set_initial_max_data"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_set_initial_max_data (config : & mut Config , v : u64 ,) { config . set_initial_max_data (v) ; }
};
}
