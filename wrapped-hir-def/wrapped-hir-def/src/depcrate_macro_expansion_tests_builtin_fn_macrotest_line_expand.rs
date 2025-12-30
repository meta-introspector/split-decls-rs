// Generated macro for test_line_expand (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_line_expand {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_line_expand"}
// Dependencies: {}
# [test] fn test_line_expand () { check (r#"
#[rustc_builtin_macro]
macro_rules! line {() => {}}

fn main() { line!() }
"# , expect ! [[r#"
#[rustc_builtin_macro]
macro_rules! line {() => {}}

fn main() { 0u32 }
"#]] ,) ; }
};
}
