// Generated macro for test_ty (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_ty {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_ty"}
// Dependencies: {}
# [test] fn test_ty () { check (r#"
macro_rules! m {
    ($t:ty) => ( fn bar() -> $t {} )
}
m! { Baz<u8> }
"# , expect ! [[r#"
macro_rules! m {
    ($t:ty) => ( fn bar() -> $t {} )
}
fn bar() -> Baz<u8> {}
"#]] ,) }
};
}
