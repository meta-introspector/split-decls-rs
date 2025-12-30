// Generated macro for test_into_iter (function)
macro_rules! Depcrate_tests_settest_into_iter {
() => {
// Module: crate::tests::set
// Provides: {"test_into_iter"}
// Dependencies: {}
# [test] fn test_into_iter () { let nums = [1 , 2 , 3] ; let set = NSSet :: from_retained_slice (& nums . map (NSNumber :: new_i32)) ; assert ! (set . into_iter () . all (| i | nums . contains (& i . as_i32 ()))) ; }
};
}
