// Generated macro for float_int_equality (function)
macro_rules! Depcrate_tests_numberfloat_int_equality {
() => {
// Module: crate::tests::number
// Provides: {"float_int_equality"}
// Dependencies: {}
# [test] fn float_int_equality () { let val1 = NSNumber :: new_f32 (1.0) ; let val2 = NSNumber :: new_u32 (1) ; let val3 = NSNumber :: new_u32 (1.0f32 . to_bits ()) ; assert_eq ! (val1 , val2) ; assert_ne ! (val1 , val3) ; }
};
}
