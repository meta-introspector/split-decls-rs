// Generated macro for test_binary_search (function)
macro_rules! Depcrate_helperstest_binary_search {
() => {
// Module: crate::helpers
// Provides: {"test_binary_search"}
// Dependencies: {}
# [test] fn test_binary_search () { struct TestCase { test_fn : fn (f64) -> bool , range : (f64 , f64) , expected : f64 , } let test_cases = [TestCase { test_fn : | x : f64 | x >= 4.0 , range : (0.0 , 10.0) , expected : 4.0 , } , TestCase { test_fn : | x : f64 | x * x >= 2.0 , range : (0.0 , 2.0) , expected : 2.0f64 . sqrt () , } , TestCase { test_fn : | x : f64 | x >= - 4.0 , range : (- 10.0 , 0.0) , expected : - 4.0 , } , TestCase { test_fn : | x : f64 | x >= 0.0 , range : (0.0 , 10.0) , expected : 0.0 , } , TestCase { test_fn : | x : f64 | x > 10.0 , range : (0.0 , 10.0) , expected : 10.0 , } ,] ; for case in test_cases { let result = binary_search (case . range . 0 , case . range . 1 , case . test_fn , 1e-4) ; assert ! ((result - case . expected) . abs () < 0.0001) ; } }
};
}
