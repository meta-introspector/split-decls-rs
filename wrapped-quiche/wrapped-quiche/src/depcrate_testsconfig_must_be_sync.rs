// Generated macro for config_must_be_sync (function)
macro_rules! Depcrate_testsconfig_must_be_sync {
() => {
// Module: crate::tests
// Provides: {"config_must_be_sync"}
// Dependencies: {}
# [rstest] fn config_must_be_sync (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut config = Config :: new (PROTOCOL_VERSION) . unwrap () ; assert_eq ! (config . set_cc_algorithm_name (cc_algorithm_name) , Ok (())) ; check_sync (& mut config) ; }
};
}
