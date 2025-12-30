// Generated macro for test_prefixes_sorted (function)
macro_rules! Depcrate_cargo_feature_nametest_prefixes_sorted {
() => {
// Module: crate::cargo::feature_name
// Provides: {"test_prefixes_sorted"}
// Dependencies: {}
# [test] fn test_prefixes_sorted () { let mut sorted_prefixes = PREFIXES ; sorted_prefixes . sort_unstable () ; assert_eq ! (PREFIXES , sorted_prefixes) ; let mut sorted_suffixes = SUFFIXES ; sorted_suffixes . sort_by (| a , b | a . bytes () . rev () . cmp (b . bytes () . rev ())) ; assert_eq ! (SUFFIXES , sorted_suffixes) ; }
};
}
