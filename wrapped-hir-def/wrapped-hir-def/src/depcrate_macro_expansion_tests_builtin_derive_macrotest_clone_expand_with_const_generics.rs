// Generated macro for test_clone_expand_with_const_generics (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_derive_macrotest_clone_expand_with_const_generics {
() => {
// Module: crate::macro_expansion_tests::builtin_derive_macro
// Provides: {"test_clone_expand_with_const_generics"}
// Dependencies: {}
# [test] fn test_clone_expand_with_const_generics () { check (r#"
//- minicore: derive, clone
#[derive(Clone)]
struct Foo<const X: usize, T>(u32);
"# , expect ! [[r#"
#[derive(Clone)]
struct Foo<const X: usize, T>(u32);

impl <const X: usize, T: $crate::clone::Clone, > $crate::clone::Clone for Foo<X, T, > where {
    fn clone(&self ) -> Self {
        match self {
            Foo(f0, )=>Foo(f0.clone(), ),
        }
    }
}"#]] ,) ; }
};
}
