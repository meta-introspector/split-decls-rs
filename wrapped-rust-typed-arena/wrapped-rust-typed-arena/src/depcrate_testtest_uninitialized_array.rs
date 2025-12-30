// Generated macro for test_uninitialized_array (function)
macro_rules! Depcrate_testtest_uninitialized_array {
() => {
// Module: crate::test
// Provides: {"test_uninitialized_array"}
// Dependencies: {}
# [test] fn test_uninitialized_array () { let arena = Arena :: with_capacity (2) ; let uninit = arena . uninitialized_array () ; arena . alloc_extend (0 .. 2) ; unsafe { for (& a , b) in (& * uninit) . iter () . zip (0 .. 2) { assert_eq ! (a . assume_init () , b) ; } assert ! ((&* arena . uninitialized_array ()) . as_ptr () != (&* uninit) . as_ptr ()) ; arena . alloc (0) ; let uninit = arena . uninitialized_array () ; assert_eq ! ((&* uninit) . len () , 3) ; } }
};
}
