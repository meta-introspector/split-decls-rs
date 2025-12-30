// Generated macro for to_snake_case_tests (module)
macro_rules! Depcrate_common_renameto_snake_case_tests {
() => {
// Module: crate::common::rename
// Provides: {"to_snake_case_tests"}
// Dependencies: {}
# [cfg (test)] mod to_snake_case_tests { use super :: to_snake_case ; # [test] fn upper () { for (input , expected) in [("abc" , "ABC") , ("a_bc" , "A_BC") , ("ABC" , "ABC") , ("A_BC" , "A_BC") , ("SomeInput" , "SOME_INPUT") , ("someInput" , "SOME_INPUT") , ("someINpuT" , "SOME_INPU_T") , ("some_INpuT" , "SOME_INPU_T") ,] { assert_eq ! (to_snake_case (input , true) , expected) ; } } # [test] fn lower () { for (input , expected) in [("abc" , "abc") , ("a_bc" , "a_bc") , ("ABC" , "abc") , ("A_BC" , "a_bc") , ("SomeInput" , "some_input") , ("someInput" , "some_input") , ("someINpuT" , "some_inpu_t") , ("some_INpuT" , "some_inpu_t") ,] { assert_eq ! (to_snake_case (input , false) , expected) ; } } }
};
}
