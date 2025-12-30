// Generated macro for test_ci_flag (function)
macro_rules! Depcrate_core_config_teststest_ci_flag {
() => {
// Module: crate::core::config::tests
// Provides: {"test_ci_flag"}
// Dependencies: {}
# [test] fn test_ci_flag () { let config = Config :: parse_inner (Flags :: parse (& ["check" . into () , "--ci=false" . into ()]) , | & _ | { toml :: from_str ("") }) ; assert ! (! config . is_running_on_ci) ; let config = Config :: parse_inner (Flags :: parse (& ["check" . into () , "--ci=true" . into ()]) , | & _ | { toml :: from_str ("") }) ; assert ! (config . is_running_on_ci) ; let config = Config :: parse_inner (Flags :: parse (& ["check" . into ()]) , | & _ | toml :: from_str ("")) ; assert_eq ! (config . is_running_on_ci , CiEnv :: is_ci ()) ; }
};
}
