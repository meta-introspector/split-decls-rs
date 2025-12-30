// Generated macro for test_pad_str_with (function)
macro_rules! Depcrate_utilstest_pad_str_with {
() => {
// Module: crate::utils
// Provides: {"test_pad_str_with"}
// Dependencies: {}
# [test] fn test_pad_str_with () { assert_eq ! (pad_str_with ("foo" , 7 , Alignment :: Center , None , '#') , "##foo##") ; assert_eq ! (pad_str_with ("foo" , 7 , Alignment :: Left , None , '#') , "foo####") ; assert_eq ! (pad_str_with ("foo" , 7 , Alignment :: Right , None , '#') , "####foo") ; assert_eq ! (pad_str_with ("foo" , 3 , Alignment :: Left , None , '#') , "foo") ; assert_eq ! (pad_str_with ("foobar" , 3 , Alignment :: Left , None , '#') , "foobar") ; assert_eq ! (pad_str_with ("foobar" , 3 , Alignment :: Left , Some ("") , '#') , "foo") ; assert_eq ! (pad_str_with ("foobarbaz" , 6 , Alignment :: Left , Some ("...") , '#') , "foo...") ; }
};
}
