// Generated macro for quiche_config_set_max_pacing_rate (function)
macro_rules! Depcrate_ffiquiche_config_set_max_pacing_rate {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_set_max_pacing_rate"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_set_max_pacing_rate (config : & mut Config , v : u64) { config . set_max_pacing_rate (v) ; }
};
}
