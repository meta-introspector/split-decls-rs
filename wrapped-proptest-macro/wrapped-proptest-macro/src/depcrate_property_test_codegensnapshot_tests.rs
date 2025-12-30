// Generated macro for snapshot_tests (module)
macro_rules! Depcrate_property_test_codegensnapshot_tests {
() => {
// Module: crate::property_test::codegen
// Provides: {"snapshot_tests"}
// Dependencies: {}
# [cfg (test)] mod snapshot_tests { use super :: * ; use syn :: parse_str ; macro_rules ! snapshot_test { ($ name : ident) => { # [test] fn $ name () { const TEXT : & str = include_str ! (concat ! ("test_data/" , stringify ! ($ name) , ".rs")) ; let tokens = generate (parse_str (TEXT) . unwrap () , $ crate :: property_test :: options :: Options :: default () ,) ; insta :: assert_debug_snapshot ! (tokens) ; } } ; } snapshot_test ! (simple) ; snapshot_test ! (many_params) ; snapshot_test ! (arg_pattern) ; snapshot_test ! (arg_ident_and_pattern) ; }
};
}
