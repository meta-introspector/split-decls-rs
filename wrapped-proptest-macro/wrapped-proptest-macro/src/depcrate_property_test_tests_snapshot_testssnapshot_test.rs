// Generated macro for snapshot_test (macro)
macro_rules! Depcrate_property_test_tests_snapshot_testssnapshot_test {
() => {
// Module: crate::property_test::tests::snapshot_tests
// Provides: {"snapshot_test"}
// Dependencies: {}
# [doc = " Helper macro to make snapshot tests"] # [doc = ""] # [doc = " The code inside the block is parsed as a function and the `#[property_test]` macro is applied"] # [doc = " to it. The generated code is then formatted, and passed to the snapshot testing library"] # [doc = ""] # [doc = " If `fails` is supplied, then the output will be"] macro_rules ! snapshot_test { ($ name : ident { $ ($ t : tt) * }) => { # [test] fn $ name () { let input = parse_quote ! { $ ($ t) * } ; let tokens = codegen :: generate (input , Options :: default ()) ; let file = syn :: parse_file (& tokens . to_string ()) . unwrap () ; let formatted = prettyplease :: unparse (& file) ; insta :: assert_snapshot ! (formatted) ; } } ; }
};
}
