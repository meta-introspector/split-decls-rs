// Generated macro for quiche_config_set_max_idle_timeout (function)
macro_rules! Depcrate_ffiquiche_config_set_max_idle_timeout {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_set_max_idle_timeout"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_set_max_idle_timeout (config : & mut Config , v : u64 ,) { config . set_max_idle_timeout (v) ; }
};
}
