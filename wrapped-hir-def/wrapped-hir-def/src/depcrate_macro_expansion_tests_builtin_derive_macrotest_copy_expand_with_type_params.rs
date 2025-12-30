// Generated macro for test_copy_expand_with_type_params (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_derive_macrotest_copy_expand_with_type_params {
() => {
// Module: crate::macro_expansion_tests::builtin_derive_macro
// Provides: {"test_copy_expand_with_type_params"}
// Dependencies: {}
# [test] fn test_copy_expand_with_type_params () { check (r#"
//- minicore: derive, copy
#[derive(Copy)]
struct Foo<A, B>;
"# , expect ! [[r#"
#[derive(Copy)]
struct Foo<A, B>;

impl <A: $crate::marker::Copy, B: $crate::marker::Copy, > $crate::marker::Copy for Foo<A, B, > where {}"#]] ,) ; }
};
}
