// Generated macro for test_get (function)
macro_rules! Depcrate_tests_settest_get {
() => {
// Module: crate::tests::set
// Provides: {"test_get"}
// Dependencies: {}
# [test] fn test_get () { let set = NSSet :: < NSString > :: new () ; assert ! (set . member (ns_string ! ("one")) . is_none ()) ; let set = NSSet :: from_slice (& [ns_string ! ("one") , ns_string ! ("two") , ns_string ! ("two")]) ; assert ! (set . member (ns_string ! ("two")) . is_some ()) ; assert ! (set . member (ns_string ! ("three")) . is_none ()) ; }
};
}
