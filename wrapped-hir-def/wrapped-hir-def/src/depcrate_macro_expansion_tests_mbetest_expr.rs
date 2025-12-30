// Generated macro for test_expr (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_expr {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_expr"}
// Dependencies: {}
# [test] fn test_expr () { check (r#"
macro_rules! m {
    ($e:expr) => { fn bar() { $e; } }
}

m! { 2 + 2 * baz(3).quux() }
"# , expect ! [[r#"
macro_rules! m {
    ($e:expr) => { fn bar() { $e; } }
}

fn bar() {
    (2+2*baz(3).quux());
}
"#]] ,) }
};
}
