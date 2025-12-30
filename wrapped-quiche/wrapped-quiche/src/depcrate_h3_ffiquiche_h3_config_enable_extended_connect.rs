// Generated macro for quiche_h3_config_enable_extended_connect (function)
macro_rules! Depcrate_h3_ffiquiche_h3_config_enable_extended_connect {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_config_enable_extended_connect"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_config_enable_extended_connect (config : & mut h3 :: Config , enabled : bool ,) { config . enable_extended_connect (enabled) ; }
};
}
