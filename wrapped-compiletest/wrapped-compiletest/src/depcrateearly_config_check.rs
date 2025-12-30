// Generated macro for early_config_check (function)
macro_rules! Depcrateearly_config_check {
() => {
// Module: crate
// Provides: {"early_config_check"}
// Dependencies: {}
pub fn early_config_check (config : & Config) { if ! config . has_html_tidy && config . mode == TestMode :: Rustdoc { warning ! ("`tidy` (html-tidy.org) is not installed; diffs will not be generated") ; } if ! config . profiler_runtime && config . mode == TestMode :: CoverageRun { let actioned = if config . bless { "blessed" } else { "checked" } ; warning ! ("profiler runtime is not available, so `.coverage` files won't be {actioned}") ; help ! ("try setting `profiler = true` in the `[build]` section of `bootstrap.toml`") ; } if env :: var ("RUST_TEST_NOCAPTURE") . is_ok () { warning ! ("`RUST_TEST_NOCAPTURE` is not supported; use the `--no-capture` flag instead") ; } }
};
}
