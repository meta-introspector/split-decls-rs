// Generated macro for test_iter (function)
macro_rules! Depcrate_tests_settest_iter {
() => {
// Module: crate::tests::set
// Provides: {"test_iter"}
// Dependencies: {}
# [test] fn test_iter () { let nums = [1 , 2 , 3] ; let set = NSSet :: from_retained_slice (& nums . map (NSNumber :: new_i32)) ; assert_eq ! (set . iter () . count () , 3) ; assert ! (set . iter () . all (| i | nums . contains (& i . as_i32 ()))) ; }
};
}
