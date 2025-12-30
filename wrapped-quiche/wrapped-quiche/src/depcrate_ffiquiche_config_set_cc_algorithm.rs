// Generated macro for quiche_config_set_cc_algorithm (function)
macro_rules! Depcrate_ffiquiche_config_set_cc_algorithm {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_set_cc_algorithm"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_set_cc_algorithm (config : & mut Config , algo : CongestionControlAlgorithm ,) { config . set_cc_algorithm (algo) ; }
};
}
