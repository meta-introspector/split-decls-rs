// Generated macro for test_string_ext (function)
macro_rules! Depcrate_stringtest_string_ext {
() => {
// Module: crate::string
// Provides: {"test_string_ext"}
// Dependencies: {}
# [cfg (feature = "std-string")] # [test] fn test_string_ext () { let mut s = String :: new () ; let t = "αβγabc" ; StringExt :: insert_str (& mut s , 0 , t) ; assert_eq ! (s , t) ; StringExt :: insert_str (& mut s , 2 , "x") ; assert_eq ! (s , "αxβγabc") ; }
};
}
