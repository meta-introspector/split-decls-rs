// Generated macro for test_option (function)
macro_rules! Depcrate_eqtest_option {
() => {
// Module: crate::eq
// Provides: {"test_option"}
// Dependencies: {}
# [test] fn test_option () { let x : Option < f32 > = None ; assert ! (x . approx_eq (None , (0.0 , 0_i32))) ; assert ! (Some (5.3_f32) . approx_eq (Some (5.3) , (0.0 , 0_i32))) ; assert ! (Some (5.3_f32) . approx_ne (Some (5.7) , (0.0 , 0_i32))) ; assert ! (Some (5.3_f32) . approx_ne (None , (0.0 , 0_i32))) ; assert ! (x . approx_ne (Some (5.3) , (0.0 , 0_i32))) ; }
};
}
