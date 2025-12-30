// Generated macro for test_meta (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_meta {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_meta"}
// Dependencies: {}
# [test] fn test_meta () { check (r#"
macro_rules! m {
    ($m:meta) => ( #[$m] fn bar() {} )
}
m! { cfg(target_os = "windows") }
m! { hello::world }
"# , expect ! [[r#"
macro_rules! m {
    ($m:meta) => ( #[$m] fn bar() {} )
}
#[cfg(target_os = "windows")] fn bar() {}
#[hello::world] fn bar() {}
"#]] ,) ; }
};
}
