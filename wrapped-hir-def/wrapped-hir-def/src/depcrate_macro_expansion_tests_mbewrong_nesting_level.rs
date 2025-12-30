// Generated macro for wrong_nesting_level (function)
macro_rules! Depcrate_macro_expansion_tests_mbewrong_nesting_level {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"wrong_nesting_level"}
// Dependencies: {}
# [test] fn wrong_nesting_level () { check (r#"
macro_rules! m {
    ($($i:ident);*) => ($i)
}
m!{a}
"# , expect ! [[r#"
macro_rules! m {
    ($($i:ident);*) => ($i)
}
/* error: expected simple binding, found nested binding `i` */
"#]] ,) ; }
};
}
