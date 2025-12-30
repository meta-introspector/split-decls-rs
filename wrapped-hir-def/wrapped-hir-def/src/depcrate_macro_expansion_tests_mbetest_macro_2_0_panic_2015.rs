// Generated macro for test_macro_2_0_panic_2015 (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_macro_2_0_panic_2015 {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_macro_2_0_panic_2015"}
// Dependencies: {}
# [test] fn test_macro_2_0_panic_2015 () { check (r#"
macro panic_2015 {
    () => (),
    (bar) => (),
}
panic_2015!(bar);
"# , expect ! [[r#"
macro panic_2015 {
    () => (),
    (bar) => (),
}

"#]] ,) ; }
};
}
