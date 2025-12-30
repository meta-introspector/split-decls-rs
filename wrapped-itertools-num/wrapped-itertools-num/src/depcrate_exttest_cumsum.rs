// Generated macro for test_cumsum (function)
macro_rules! Depcrate_exttest_cumsum {
() => {
// Module: crate::ext
// Provides: {"test_cumsum"}
// Dependencies: {}
# [test] fn test_cumsum () { let data = [1. , 2. , 3.] ; let mut iter = data . iter () . cumsum :: < f64 > () ; assert_eq ! (iter . next () , Some (1.)) ; assert_eq ! (iter . next () , Some (3.)) ; assert_eq ! (iter . next () , Some (6.)) ; }
};
}
