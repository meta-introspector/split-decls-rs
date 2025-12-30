// Generated macro for test_macro_3 (function)
macro_rules! Depcrate_macrostest_macro_3 {
() => {
// Module: crate::macros
// Provides: {"test_macro_3"}
// Dependencies: {}
# [test] fn test_macro_3 () { use crate :: F32Margin ; let a : f32 = 0.15 + 0.15 + 0.15 ; let b : f32 = 0.1 + 0.1 + 0.25 ; assert ! (approx_eq ! (f32 , a , b , F32Margin { epsilon : 0.0 , ulps : 2 })) ; assert ! (approx_eq ! (f32 , a , b , F32Margin :: default ())) ; assert_approx_eq ! (f32 , a , b , F32Margin { epsilon : 0.0 , ulps : 2 }) ; assert_approx_eq ! (f32 , a , b , F32Margin :: default ()) ; }
};
}
