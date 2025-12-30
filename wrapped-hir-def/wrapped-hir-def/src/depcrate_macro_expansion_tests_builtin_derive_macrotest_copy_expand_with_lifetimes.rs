// Generated macro for test_copy_expand_with_lifetimes (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_derive_macrotest_copy_expand_with_lifetimes {
() => {
// Module: crate::macro_expansion_tests::builtin_derive_macro
// Provides: {"test_copy_expand_with_lifetimes"}
// Dependencies: {}
# [test] fn test_copy_expand_with_lifetimes () { check (r#"
//- minicore: derive, copy
#[derive(Copy)]
struct Foo<A, B, 'a, 'b>;
"# , expect ! [[r#"
#[derive(Copy)]
struct Foo<A, B, 'a, 'b>;

impl <A: $crate::marker::Copy, B: $crate::marker::Copy, > $crate::marker::Copy for Foo<A, B, > where {}"#]] ,) ; }
};
}
