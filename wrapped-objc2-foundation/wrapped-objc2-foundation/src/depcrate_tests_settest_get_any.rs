// Generated macro for test_get_any (function)
macro_rules! Depcrate_tests_settest_get_any {
() => {
// Module: crate::tests::set
// Provides: {"test_get_any"}
// Dependencies: {}
# [test] fn test_get_any () { let set = NSSet :: < NSObject > :: new () ; assert ! (set . anyObject () . is_none ()) ; let strs = [ns_string ! ("one") , ns_string ! ("two") , ns_string ! ("three")] ; let set = NSSet :: from_slice (& strs) ; let any = set . anyObject () . unwrap () ; assert ! (&* any == strs [0] || &* any == strs [1] || &* any == strs [2]) ; }
};
}
