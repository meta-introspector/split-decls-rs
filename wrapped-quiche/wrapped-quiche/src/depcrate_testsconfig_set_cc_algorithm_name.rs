// Generated macro for config_set_cc_algorithm_name (function)
macro_rules! Depcrate_testsconfig_set_cc_algorithm_name {
() => {
// Module: crate::tests
// Provides: {"config_set_cc_algorithm_name"}
// Dependencies: {}
# [test] fn config_set_cc_algorithm_name () { let mut config = Config :: new (PROTOCOL_VERSION) . unwrap () ; assert_eq ! (config . set_cc_algorithm_name ("reno") , Ok (())) ; assert_eq ! (config . set_cc_algorithm_name ("???") , Err (Error :: CongestionControl)) ; }
};
}
