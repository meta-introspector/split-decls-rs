// Generated macro for test_tt_composite2 (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_tt_composite2 {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_tt_composite2"}
// Dependencies: {}
# [test] fn test_tt_composite2 () { check (r#"
macro_rules! m { ($($tt:tt)*) => { abs!(=> $($tt)*); } }
m! {#}
"# , expect ! [[r#"
macro_rules! m { ($($tt:tt)*) => { abs!(=> $($tt)*); } }
abs!( = > #);
"#]] ,) ; }
};
}
