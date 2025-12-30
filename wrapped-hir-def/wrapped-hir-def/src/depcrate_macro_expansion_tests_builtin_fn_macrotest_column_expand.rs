// Generated macro for test_column_expand (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_column_expand {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_column_expand"}
// Dependencies: {}
# [test] fn test_column_expand () { check (r#"
#[rustc_builtin_macro]
macro_rules! column {() => {}}

fn main() { column!(); }
"# , expect ! [[r#"
#[rustc_builtin_macro]
macro_rules! column {() => {}}

fn main() { 0u32; }
"#]] ,) ; }
};
}
