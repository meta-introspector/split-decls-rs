// Generated macro for test_stringify_expand (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_stringify_expand {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_stringify_expand"}
// Dependencies: {}
# [test] fn test_stringify_expand () { check (r#"
#[rustc_builtin_macro]
macro_rules! stringify {() => {}}

fn main() {
    stringify!(
        a
        b
        c
    );
}
"# , expect ! [[r##"
#[rustc_builtin_macro]
macro_rules! stringify {() => {}}

fn main() {
    "a b c";
}
"##]] ,) ; }
};
}
