// Generated macro for test_to_camel_case (function)
macro_rules! Depcrate_utiltest_to_camel_case {
() => {
// Module: crate::util
// Provides: {"test_to_camel_case"}
// Dependencies: {}
# [test] fn test_to_camel_case () { assert_eq ! (& to_camel_case ("test") [..] , "test") ; assert_eq ! (& to_camel_case ("_test") [..] , "test") ; assert_eq ! (& to_camel_case ("first_second") [..] , "firstSecond") ; assert_eq ! (& to_camel_case ("first_") [..] , "first") ; assert_eq ! (& to_camel_case ("a_b_c") [..] , "aBC") ; assert_eq ! (& to_camel_case ("a_bc") [..] , "aBc") ; assert_eq ! (& to_camel_case ("a_b") [..] , "aB") ; assert_eq ! (& to_camel_case ("a") [..] , "a") ; assert_eq ! (& to_camel_case ("") [..] , "") ; }
};
}
