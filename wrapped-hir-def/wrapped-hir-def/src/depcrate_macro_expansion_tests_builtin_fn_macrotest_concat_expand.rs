// Generated macro for test_concat_expand (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_concat_expand {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_concat_expand"}
// Dependencies: {}
# [test] fn test_concat_expand () { check (r##"
#[rustc_builtin_macro]
macro_rules! concat {}

fn main() { concat!("fo", "o", 0, r#""bar""#, "\n", false, '"', -4, - 4, '\0'); }
"## , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! concat {}

fn main() { "foo0\"bar\"\nfalse\"-4-4\u{0}"; }
"##]] ,) ; }
};
}
