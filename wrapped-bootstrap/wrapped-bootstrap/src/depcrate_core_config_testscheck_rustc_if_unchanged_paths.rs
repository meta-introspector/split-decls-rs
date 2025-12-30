// Generated macro for check_rustc_if_unchanged_paths (function)
macro_rules! Depcrate_core_config_testscheck_rustc_if_unchanged_paths {
() => {
// Module: crate::core::config::tests
// Provides: {"check_rustc_if_unchanged_paths"}
// Dependencies: {}
# [test] fn check_rustc_if_unchanged_paths () { let config = parse ("") ; let normalised_allowed_paths : Vec < _ > = RUSTC_IF_UNCHANGED_ALLOWED_PATHS . iter () . map (| t | { t . strip_prefix (":!") . expect (& format ! ("{t} doesn't have ':!' prefix, but it should.")) }) . collect () ; for p in normalised_allowed_paths { assert ! (config . src . join (p) . exists () , "{p} doesn't exist.") ; } }
};
}
