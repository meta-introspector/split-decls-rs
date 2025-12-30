// Generated macro for tests (module)
macro_rules! Depcrate_dynamic_type_reftests {
() => {
// Module: crate::dynamic::type_ref
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn create () { assert_eq ! (TypeRef :: named ("MyObj") . to_string () , "MyObj") ; assert_eq ! (TypeRef :: named_nn ("MyObj") . to_string () , "MyObj!") ; assert_eq ! (TypeRef :: named_list ("MyObj") . to_string () , "[MyObj]") ; assert_eq ! (TypeRef :: named_list_nn ("MyObj") . to_string () , "[MyObj]!") ; assert_eq ! (TypeRef :: named_nn_list ("MyObj") . to_string () , "[MyObj!]") ; assert_eq ! (TypeRef :: named_nn_list_nn ("MyObj") . to_string () , "[MyObj!]!") ; } }
};
}
