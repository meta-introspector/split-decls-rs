// Generated macro for test_literal (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_literal {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_literal"}
// Dependencies: {}
# [test] fn test_literal () { check (r#"
macro_rules! m {
    ($type:ty, $lit:literal) => { const VALUE: $type = $ lit; };
}
m!(u8, 0);
"# , expect ! [[r#"
macro_rules! m {
    ($type:ty, $lit:literal) => { const VALUE: $type = $ lit; };
}
const VALUE: u8 = 0;
"#]] ,) ; check (r#"
macro_rules! m {
    ($type:ty, $lit:literal) => { const VALUE: $ type = $ lit; };
}
m!(i32, -1);
"# , expect ! [[r#"
macro_rules! m {
    ($type:ty, $lit:literal) => { const VALUE: $ type = $ lit; };
}
const VALUE: i32 = -1;
"#]] ,) ; }
};
}
