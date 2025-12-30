// Generated macro for test_copy_expand_in_core (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_derive_macrotest_copy_expand_in_core {
() => {
// Module: crate::macro_expansion_tests::builtin_derive_macro
// Provides: {"test_copy_expand_in_core"}
// Dependencies: {}
# [test] fn test_copy_expand_in_core () { check (r#"
//- /lib.rs crate:core
#[rustc_builtin_macro]
macro derive {}
#[rustc_builtin_macro]
macro Copy {}
#[derive(Copy)]
struct Foo;
"# , expect ! [[r#"
#[rustc_builtin_macro]
macro derive {}
#[rustc_builtin_macro]
macro Copy {}
#[derive(Copy)]
struct Foo;

impl <> $crate::marker::Copy for Foo< > where {}"#]] ,) ; }
};
}
