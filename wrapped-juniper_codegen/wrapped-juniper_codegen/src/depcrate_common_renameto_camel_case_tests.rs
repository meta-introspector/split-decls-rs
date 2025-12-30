// Generated macro for to_camel_case_tests (module)
macro_rules! Depcrate_common_renameto_camel_case_tests {
() => {
// Module: crate::common::rename
// Provides: {"to_camel_case_tests"}
// Dependencies: {}
# [cfg (test)] mod to_camel_case_tests { use super :: to_camel_case ; # [test] fn camel () { for (input , expected) in [("test" , "test") , ("_test" , "test") , ("__test" , "__test") , ("first_second" , "firstSecond") , ("first_" , "first") , ("a_b_c" , "aBC") , ("a_bc" , "aBc") , ("a_b" , "aB") , ("a" , "a") , ("" , "") ,] { assert_eq ! (to_camel_case (input) , expected) ; } } }
};
}
