// Generated macro for test_stmt (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_stmt {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_stmt"}
// Dependencies: {}
# [test] fn test_stmt () { check (r#"
macro_rules! m {
    ($s:stmt) => ( fn bar() { $s; } )
}
m! { 2 }
m! { let a = 0 }
"# , expect ! [[r#"
macro_rules! m {
    ($s:stmt) => ( fn bar() { $s; } )
}
fn bar() {
    2;
}
fn bar() {
    let a = 0;
}
"#]] ,) }
};
}
