// Generated macro for parse_rust_std_features_empty (function)
macro_rules! Depcrate_core_config_testsparse_rust_std_features_empty {
() => {
// Module: crate::core::config::tests
// Provides: {"parse_rust_std_features_empty"}
// Dependencies: {}
# [test] fn parse_rust_std_features_empty () { let config = parse ("rust.std-features = []") ; let expected_features : BTreeSet < String > = BTreeSet :: new () ; assert_eq ! (config . rust_std_features , expected_features) ; }
};
}
