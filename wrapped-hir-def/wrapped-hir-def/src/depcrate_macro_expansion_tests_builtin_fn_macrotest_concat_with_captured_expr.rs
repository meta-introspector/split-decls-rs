// Generated macro for test_concat_with_captured_expr (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_concat_with_captured_expr {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_concat_with_captured_expr"}
// Dependencies: {}
# [test] fn test_concat_with_captured_expr () { check (r##"
#[rustc_builtin_macro]
macro_rules! concat {}

macro_rules! surprise {
    () => { "s" };
}

fn main() { concat!(surprise!()); }
"## , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! concat {}

macro_rules! surprise {
    () => { "s" };
}

fn main() { "s"; }
"##]] ,) ; }
};
}
