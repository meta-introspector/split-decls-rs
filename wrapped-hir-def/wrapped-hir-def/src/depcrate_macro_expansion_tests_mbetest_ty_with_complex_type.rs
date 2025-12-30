// Generated macro for test_ty_with_complex_type (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_ty_with_complex_type {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_ty_with_complex_type"}
// Dependencies: {}
# [test] fn test_ty_with_complex_type () { check (r#"
macro_rules! m {
    ($t:ty) => ( fn bar() -> $ t {} )
}

m! { &'a Baz<u8> }

m! { extern "Rust" fn() -> Ret }
"# , expect ! [[r#"
macro_rules! m {
    ($t:ty) => ( fn bar() -> $ t {} )
}

fn bar() -> &'a Baz<u8> {}

fn bar() -> extern "Rust" fn() -> Ret {}
"#]] ,) ; }
};
}
