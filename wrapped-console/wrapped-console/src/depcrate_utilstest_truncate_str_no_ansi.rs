// Generated macro for test_truncate_str_no_ansi (function)
macro_rules! Depcrate_utilstest_truncate_str_no_ansi {
() => {
// Module: crate::utils
// Provides: {"test_truncate_str_no_ansi"}
// Dependencies: {}
# [test] fn test_truncate_str_no_ansi () { assert_eq ! (& truncate_str ("foo bar" , 7 , "!") , "foo bar") ; assert_eq ! (& truncate_str ("foo bar" , 5 , "") , "foo b") ; assert_eq ! (& truncate_str ("foo bar" , 5 , "!") , "foo !") ; assert_eq ! (& truncate_str ("foo bar baz" , 10 , "...") , "foo bar...") ; assert_eq ! (& truncate_str ("foo bar" , 0 , "") , "") ; assert_eq ! (& truncate_str ("foo bar" , 0 , "!") , "!") ; assert_eq ! (& truncate_str ("foo bar" , 2 , "!!!") , "!!!") ; assert_eq ! (& truncate_str ("ab" , 2 , "!!!") , "ab") ; }
};
}
