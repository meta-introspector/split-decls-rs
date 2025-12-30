// Generated macro for tests (module)
macro_rules! Depcrate_util_path_to_stringtests {
() => {
// Module: crate::util::path_to_string
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use syn :: parse_quote ; use super :: path_to_string ; # [test] fn simple_ident () { assert_eq ! (path_to_string (& parse_quote ! (a)) , "a") ; } # [test] fn simple_path () { assert_eq ! (path_to_string (& parse_quote ! (a :: b)) , "a::b") ; } }
};
}
