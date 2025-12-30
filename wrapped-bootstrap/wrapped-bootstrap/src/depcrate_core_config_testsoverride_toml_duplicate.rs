// Generated macro for override_toml_duplicate (function)
macro_rules! Depcrate_core_config_testsoverride_toml_duplicate {
() => {
// Module: crate::core::config::tests
// Provides: {"override_toml_duplicate"}
// Dependencies: {}
# [test] # [should_panic] fn override_toml_duplicate () { Config :: parse_inner (Flags :: parse (& ["check" . to_owned () , "--config=/does/not/exist" . to_string () , "--set=change-id=1" . to_owned () , "--set=change-id=2" . to_owned () ,]) , | & _ | toml :: from_str ("change-id = 0") ,) ; }
};
}
