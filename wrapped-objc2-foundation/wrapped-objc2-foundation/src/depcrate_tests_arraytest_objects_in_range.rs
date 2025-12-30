// Generated macro for test_objects_in_range (function)
macro_rules! Depcrate_tests_arraytest_objects_in_range {
() => {
// Module: crate::tests::array
// Provides: {"test_objects_in_range"}
// Dependencies: {}
# [test] fn test_objects_in_range () { let array = sample_array (4) ; let middle_objs = array . objects_in_range (1 .. 3) ; assert_eq ! (middle_objs . len () , 2) ; assert_eq ! (middle_objs [0] , array . objectAtIndex (1)) ; assert_eq ! (middle_objs [1] , array . objectAtIndex (2)) ; let empty_objs = array . objects_in_range (1 .. 1) ; assert ! (empty_objs . is_empty ()) ; let all_objs = array . objects_in_range (0 .. 4) ; assert_eq ! (all_objs . len () , 4) ; }
};
}
