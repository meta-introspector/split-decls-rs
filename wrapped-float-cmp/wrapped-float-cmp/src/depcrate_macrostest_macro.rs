// Generated macro for test_macro (function)
macro_rules! Depcrate_macrostest_macro {
() => {
// Module: crate::macros
// Provides: {"test_macro"}
// Dependencies: {}
# [test] fn test_macro () { let a : f32 = 0.15 + 0.15 + 0.15 ; let b : f32 = 0.1 + 0.1 + 0.25 ; assert ! (approx_eq ! (f32 , a , b)) ; assert ! (approx_eq ! (f32 , a , b , ulps = 2)) ; assert ! (approx_eq ! (f32 , a , b , epsilon = 0.00000003)) ; assert ! (approx_eq ! (f32 , a , b , epsilon = 0.00000003 , ulps = 2)) ; assert ! (approx_eq ! (f32 , a , b , (0.0 , 2))) ; assert_approx_eq ! (f32 , a , b) ; assert_approx_eq ! (f32 , a , b , ulps = 2) ; assert_approx_eq ! (f32 , a , b , epsilon = 0.00000003) ; assert_approx_eq ! (f32 , a , b , epsilon = 0.00000003 , ulps = 2) ; assert_approx_eq ! (f32 , a , b , (0.0 , 2)) ; }
};
}
