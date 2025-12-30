// Generated macro for default_lld_opt_in_targets (function)
macro_rules! Depcrate_core_config_toml_rustdefault_lld_opt_in_targets {
() => {
// Module: crate::core::config::toml::rust
// Provides: {"default_lld_opt_in_targets"}
// Dependencies: {}
# [cfg (test)] pub fn default_lld_opt_in_targets () -> Vec < String > { TEST_LLD_OPT_IN_TARGETS . with (| cell | cell . borrow () . clone ()) . unwrap_or_default () }
};
}
