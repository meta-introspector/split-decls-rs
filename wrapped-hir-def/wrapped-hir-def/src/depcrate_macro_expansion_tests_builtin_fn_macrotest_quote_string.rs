// Generated macro for test_quote_string (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_quote_string {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_quote_string"}
// Dependencies: {}
# [test] fn test_quote_string () { check (r##"
#[rustc_builtin_macro]
macro_rules! stringify {}

fn main() { stringify!("hello"); }
"## , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! stringify {}

fn main() { "\"hello\""; }
"##]] ,) ; }
};
}
