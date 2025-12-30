// Generated macro for test_get (function)
macro_rules! Depcrate_tests_arraytest_get {
() => {
// Module: crate::tests::array
// Provides: {"test_get"}
// Dependencies: {}
# [test] fn test_get () { let array = sample_array (4) ; assert_ne ! (array . objectAtIndex (0) , array . objectAtIndex (3)) ; assert_eq ! (array . firstObject () . unwrap () , array . objectAtIndex (0)) ; assert_eq ! (array . lastObject () . unwrap () , array . objectAtIndex (3)) ; let empty_array = < NSArray < NSObject > > :: new () ; assert ! (empty_array . firstObject () . is_none ()) ; assert ! (empty_array . lastObject () . is_none ()) ; }
};
}
