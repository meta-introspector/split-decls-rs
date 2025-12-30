// Generated macro for contextualize_config (function)
macro_rules! Depcrate_test_runner_configcontextualize_config {
() => {
// Module: crate::test_runner::config
// Provides: {"contextualize_config"}
// Dependencies: {}
# [doc = " Without the `std` feature this function returns config unchanged."] # [cfg (not (all (feature = "std" , not (target_arch = "wasm32"))))] pub fn contextualize_config (result : Config) -> Config { result }
};
}
