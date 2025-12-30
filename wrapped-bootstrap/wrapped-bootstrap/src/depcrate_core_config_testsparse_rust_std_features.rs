// Generated macro for parse_rust_std_features (function)
macro_rules! Depcrate_core_config_testsparse_rust_std_features {
() => {
// Module: crate::core::config::tests
// Provides: {"parse_rust_std_features"}
// Dependencies: {}
# [test] fn parse_rust_std_features () { let config = parse ("rust.std-features = [\"panic-unwind\", \"backtrace\"]") ; let expected_features : BTreeSet < String > = ["panic-unwind" , "backtrace"] . into_iter () . map (| s | s . to_string ()) . collect () ; assert_eq ! (config . rust_std_features , expected_features) ; }
};
}
