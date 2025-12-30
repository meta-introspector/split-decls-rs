// Generated macro for quiche_config_set_max_amplification_factor (function)
macro_rules! Depcrate_ffiquiche_config_set_max_amplification_factor {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_set_max_amplification_factor"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_set_max_amplification_factor (config : & mut Config , v : usize ,) { config . set_max_amplification_factor (v) ; }
};
}
