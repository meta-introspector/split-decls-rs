// Generated macro for test_len (function)
macro_rules! Depcrate_tests_settest_len {
() => {
// Module: crate::tests::set
// Provides: {"test_len"}
// Dependencies: {}
# [test] fn test_len () { let set = NSSet :: < NSObject > :: new () ; assert ! (set . is_empty ()) ; let set = NSSet :: from_slice (& [ns_string ! ("one") , ns_string ! ("two") , ns_string ! ("two")]) ; assert_eq ! (set . len () , 2) ; let set = NSSet :: from_retained_slice (& [NSNumber :: new_i32 (1) , NSNumber :: new_i32 (2) , NSNumber :: new_i32 (3) ,]) ; assert_eq ! (set . len () , 3) ; }
};
}
