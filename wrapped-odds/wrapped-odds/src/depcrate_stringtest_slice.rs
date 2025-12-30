// Generated macro for test_slice (function)
macro_rules! Depcrate_stringtest_slice {
() => {
// Module: crate::string
// Provides: {"test_slice"}
// Dependencies: {}
# [allow (deprecated)] # [test] fn test_slice () { let t = "αβγabc" ; assert_eq ! (t . get_slice (..) , Some (t)) ; assert_eq ! (t . get_slice (0 .. t . len ()) , Some (t)) ; assert_eq ! (t . get_slice (1 ..) , None) ; assert_eq ! (t . get_slice (0 .. t . len () + 1) , None) ; assert_eq ! (t . get_slice (t . len () + 1 ..) , None) ; assert_eq ! (t . get_slice (t . len () ..) , Some ("")) ; }
};
}
