// Generated macro for config_version_invalid (function)
macro_rules! Depcrate_testsconfig_version_invalid {
() => {
// Module: crate::tests
// Provides: {"config_version_invalid"}
// Dependencies: {}
# [test] fn config_version_invalid () { assert_eq ! (Config :: new (0xb1bababa) . err () . unwrap () , Error :: UnknownVersion) ; }
};
}
