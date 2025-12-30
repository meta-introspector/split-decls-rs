// Generated macro for test_zero_cap (function)
macro_rules! Depcrate_testtest_zero_cap {
() => {
// Module: crate::test
// Provides: {"test_zero_cap"}
// Dependencies: {}
# [test] fn test_zero_cap () { let arena = Arena :: with_capacity (0) ; let a = arena . alloc (1) ; let b = arena . alloc (2) ; assert_eq ! (* a , 1) ; assert_eq ! (* b , 2) ; assert_eq ! (arena . len () , 2) ; }
};
}
