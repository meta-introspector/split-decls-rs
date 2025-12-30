// Generated macro for test_copy_expand_simple (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_derive_macrotest_copy_expand_simple {
() => {
// Module: crate::macro_expansion_tests::builtin_derive_macro
// Provides: {"test_copy_expand_simple"}
// Dependencies: {}
# [test] fn test_copy_expand_simple () { check (r#"
//- minicore: derive, copy
#[derive(Copy)]
struct Foo;
"# , expect ! [[r#"
#[derive(Copy)]
struct Foo;

impl <> $crate::marker::Copy for Foo< > where {}"#]] ,) ; }
};
}
