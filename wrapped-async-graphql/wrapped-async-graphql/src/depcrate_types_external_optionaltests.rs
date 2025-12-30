// Generated macro for tests (module)
macro_rules! Depcrate_types_external_optionaltests {
() => {
// Module: crate::types::external::optional
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: InputType ; # [test] fn test_optional_type () { assert_eq ! (Option ::< i32 >:: type_name () , "Int") ; assert_eq ! (Option ::< i32 >:: qualified_type_name () , "Int") ; assert_eq ! (& Option ::< i32 >:: type_name () , "Int") ; assert_eq ! (& Option ::< i32 >:: qualified_type_name () , "Int") ; } }
};
}
