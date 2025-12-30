// Generated macro for test_join (function)
macro_rules! Depcratetest_join {
() => {
// Module: crate
// Provides: {"test_join"}
// Dependencies: {}
# [test] # [allow (deprecated)] fn test_join () { assert_eq ! (join (vec ! []) , "") ; assert_eq ! (join (vec ! [""]) , "''") ; assert_eq ! (join (vec ! ["a" , "b"]) , "a b") ; assert_eq ! (join (vec ! ["foo bar" , "baz"]) , "'foo bar' baz") ; }
};
}
