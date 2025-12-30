// Generated macro for test_lifetime (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_lifetime {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_lifetime"}
// Dependencies: {}
# [test] fn test_lifetime () { check (r#"
macro_rules! m {
    ($lt:lifetime) => { struct Ref<$lt>{ s: &$ lt str } }
}
m! {'a}
"# , expect ! [[r#"
macro_rules! m {
    ($lt:lifetime) => { struct Ref<$lt>{ s: &$ lt str } }
}
struct Ref<'a> {
    s: &'a str
}
"#]] ,) ; }
};
}
