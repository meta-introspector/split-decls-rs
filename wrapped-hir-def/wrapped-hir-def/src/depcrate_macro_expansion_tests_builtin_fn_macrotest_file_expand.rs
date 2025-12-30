// Generated macro for test_file_expand (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_file_expand {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_file_expand"}
// Dependencies: {}
# [test] fn test_file_expand () { check (r#"
#[rustc_builtin_macro]
macro_rules! file {() => {}}

fn main() { file!(); }
"# , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! file {() => {}}

fn main() { "file"; }
"##]] ,) ; }
};
}
