// Generated macro for test_invert_angular (function)
macro_rules! Depcrate_helperstest_invert_angular {
() => {
// Module: crate::helpers
// Provides: {"test_invert_angular"}
// Dependencies: {}
# [test] fn test_invert_angular () { struct TestCase { f : fn (f64) -> f64 , y : f64 , r : (f64 , f64) , expected : f64 , } fn f1 (x : f64) -> f64 { (2.0 * x) . rem_euclid (360.0) } fn f2 (x : f64) -> f64 { (3.0 * x) . rem_euclid (360.0) } fn f3 (x : f64) -> f64 { (x) . rem_euclid (360.0) } let tolerance = 1e-5 ; let test_cases = [TestCase { f : f1 , y : 4.0 , r : (0.0 , 10.0) , expected : 4.0 , } , TestCase { f : f2 , y : 6.0 , r : (0.0 , 20.0) , expected : 6.0 , } , TestCase { f : f3 , y : 400.0 , r : (0.0 , 10.0) , expected : 10.0 , } , TestCase { f : f3 , y : 0.0 , r : (0.0 , 10.0) , expected : 0.0 , } , TestCase { f : f3 , y : 10.0 , r : (0.0 , 10.0) , expected : 10.0 , } ,] ; for case in test_cases { let x = invert_angular (case . f , case . y , case . r) ; assert ! ((((case . f) (x)) . rem_euclid (360.0) - case . expected) . abs () < tolerance) ; } }
};
}
