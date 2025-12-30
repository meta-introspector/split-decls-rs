// Generated macro for with_lld_opt_in_targets (function)
macro_rules! Depcrate_core_config_toml_rustwith_lld_opt_in_targets {
() => {
// Module: crate::core::config::toml::rust
// Provides: {"with_lld_opt_in_targets"}
// Dependencies: {}
# [cfg (test)] pub fn with_lld_opt_in_targets < R > (targets : Vec < String > , f : impl FnOnce () -> R) -> R { TEST_LLD_OPT_IN_TARGETS . with (| cell | { let prev = cell . replace (Some (targets)) ; let result = f () ; cell . replace (prev) ; result }) }
};
}
