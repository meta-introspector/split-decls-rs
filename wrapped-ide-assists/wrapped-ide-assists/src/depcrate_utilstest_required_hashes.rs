// Generated macro for test_required_hashes (function)
macro_rules! Depcrate_utilstest_required_hashes {
() => {
// Module: crate::utils
// Provides: {"test_required_hashes"}
// Dependencies: {}
# [test] fn test_required_hashes () { assert_eq ! (0 , required_hashes ("abc")) ; assert_eq ! (0 , required_hashes ("###")) ; assert_eq ! (1 , required_hashes ("\"")) ; assert_eq ! (2 , required_hashes ("\"#abc")) ; assert_eq ! (0 , required_hashes ("#abc")) ; assert_eq ! (3 , required_hashes ("#ab\"##c")) ; assert_eq ! (5 , required_hashes ("#ab\"##\"####c")) ; }
};
}
