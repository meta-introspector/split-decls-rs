// Generated macro for normalize_args (function)
macro_rules! Depcrate_core_config_flagsnormalize_args {
() => {
// Module: crate::core::config::flags
// Provides: {"normalize_args"}
// Dependencies: {}
fn normalize_args (args : & [String]) -> Vec < String > { let first = String :: from ("x.py") ; let it = std :: iter :: once (first) . chain (args . iter () . cloned ()) ; it . collect () }
};
}
