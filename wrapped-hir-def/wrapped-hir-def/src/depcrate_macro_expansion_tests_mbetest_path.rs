// Generated macro for test_path (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_path {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_path"}
// Dependencies: {}
# [test] fn test_path () { check (r#"
macro_rules! m {
    ($p:path) => { fn foo() { let a = $p; } }
}

m! { foo }

m! { bar::<u8>::baz::<u8> }
"# , expect ! [[r#"
macro_rules! m {
    ($p:path) => { fn foo() { let a = $p; } }
}

fn foo() {
    let a = foo;
}

fn foo() {
    let a = bar::<u8>::baz::<u8> ;
}
"#]] ,) ; }
};
}
