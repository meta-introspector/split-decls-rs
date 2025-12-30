// Generated macro for tests (module)
macro_rules! Depcrate_utilstests {
() => {
// Module: crate::utils
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_split_args () { assert ! (split_args ("\"tada") . is_err ()) ; assert ! (split_args ("\'tada") . is_err ()) ; assert_eq ! (split_args ("a \"b\" c") , Ok (vec ! ["a" . to_string () , "\"b\"" . to_string () , "c" . to_string ()])) ; assert_eq ! (split_args ("    a    \"b\" c    ") , Ok (vec ! ["a" . to_string () , "\"b\"" . to_string () , "c" . to_string ()])) ; } }
};
}
