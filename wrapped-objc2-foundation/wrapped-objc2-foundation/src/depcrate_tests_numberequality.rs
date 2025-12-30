// Generated macro for equality (function)
macro_rules! Depcrate_tests_numberequality {
() => {
// Module: crate::tests::number
// Provides: {"equality"}
// Dependencies: {}
# [test] fn equality () { let val1 = NSNumber :: new_u32 (123) ; let val2 = NSNumber :: new_u32 (123) ; let val3 = NSNumber :: new_u8 (123) ; assert_eq ! (val1 , val1) ; assert_eq ! (val1 , val2) ; assert_eq ! (val1 , val3) ; let val4 = NSNumber :: new_u32 (456) ; assert_ne ! (val1 , val4) ; }
};
}
