// Generated macro for test_to_path_buf (function)
macro_rules! Depcrate_teststest_to_path_buf {
() => {
// Module: crate::tests
// Provides: {"test_to_path_buf"}
// Dependencies: {}
# [test] fn test_to_path_buf () { let path = rp ("/hello///world//") ; let path_buf = path . to_path (".") ; let expected = Path :: new (".") . join ("hello") . join ("world") ; assert_eq ! (expected , path_buf) ; }
};
}
