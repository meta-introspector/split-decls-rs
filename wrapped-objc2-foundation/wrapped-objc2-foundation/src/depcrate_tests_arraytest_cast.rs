// Generated macro for test_cast (function)
macro_rules! Depcrate_tests_arraytest_cast {
() => {
// Module: crate::tests::array
// Provides: {"test_cast"}
// Dependencies: {}
# [test] fn test_cast () { let array = NSArray :: from_retained_slice (& [NSNumber :: new_i64 (42)]) ; let array = unsafe { array . cast_unchecked :: < NSValue > () } ; let value = array . objectAtIndex (0) ; assert_eq ! (unsafe { value . get ::< i64 > () } , 42) ; }
};
}
