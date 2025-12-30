// Generated macro for test_intersection (function)
macro_rules! Depcrate_tests_settest_intersection {
() => {
// Module: crate::tests::set
// Provides: {"test_intersection"}
// Dependencies: {}
# [test] fn test_intersection () { let set1 = NSSet :: from_slice (& [ns_string ! ("one") , ns_string ! ("two")]) ; let set2 = NSSet :: from_slice (& [ns_string ! ("one") , ns_string ! ("two") , ns_string ! ("three")]) ; let set3 = NSSet :: from_slice (& [ns_string ! ("four") , ns_string ! ("five") , ns_string ! ("six")]) ; assert ! (set1 . intersectsSet (& set2)) ; assert ! (! set1 . intersectsSet (& set3)) ; assert ! (! set2 . intersectsSet (& set3)) ; }
};
}
