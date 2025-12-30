// Generated macro for test_pad_str (function)
macro_rules! Depcrate_utilstest_pad_str {
() => {
// Module: crate::utils
// Provides: {"test_pad_str"}
// Dependencies: {}
# [test] fn test_pad_str () { assert_eq ! (pad_str ("foo" , 7 , Alignment :: Center , None) , "  foo  ") ; assert_eq ! (pad_str ("foo" , 7 , Alignment :: Left , None) , "foo    ") ; assert_eq ! (pad_str ("foo" , 7 , Alignment :: Right , None) , "    foo") ; assert_eq ! (pad_str ("foo" , 3 , Alignment :: Left , None) , "foo") ; assert_eq ! (pad_str ("foobar" , 3 , Alignment :: Left , None) , "foobar") ; assert_eq ! (pad_str ("foobar" , 3 , Alignment :: Left , Some ("")) , "foo") ; assert_eq ! (pad_str ("foobarbaz" , 6 , Alignment :: Left , Some ("...")) , "foo...") ; }
};
}
