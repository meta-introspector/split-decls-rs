// Generated macro for quiche_config_set_disable_active_migration (function)
macro_rules! Depcrate_ffiquiche_config_set_disable_active_migration {
() => {
// Module: crate::ffi
// Provides: {"quiche_config_set_disable_active_migration"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_config_set_disable_active_migration (config : & mut Config , v : bool ,) { config . set_disable_active_migration (v) ; }
};
}
