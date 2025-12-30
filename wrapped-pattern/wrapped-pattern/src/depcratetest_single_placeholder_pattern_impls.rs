// Generated macro for test_single_placeholder_pattern_impls (function)
macro_rules! Depcratetest_single_placeholder_pattern_impls {
() => {
// Module: crate
// Provides: {"test_single_placeholder_pattern_impls"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn test_single_placeholder_pattern_impls () { let a = SinglePlaceholderPattern :: try_from_str ("{0}" , Default :: default ()) . unwrap () ; let b = SinglePlaceholderPattern :: try_from_str ("{0}" , Default :: default ()) . unwrap () ; assert_eq ! (a , b) ; let c = b . clone () ; assert_eq ! (a , c) ; }
};
}
