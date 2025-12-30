// Generated macro for test_edition_handling_in (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_edition_handling_in {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_edition_handling_in"}
// Dependencies: {}
# [test] fn test_edition_handling_in () { check (r#"
//- /main.rs crate:main deps:old edition:2021
fn f() {
    old::parse_try_old!(try!{});
}
//- /old.rs crate:old edition:2015
#[macro_export]
macro_rules! parse_try_old {
    ($it:expr) => {};
}
 "# , expect ! [[r#"
fn f() {
    ;
}
"#]] ,) ; }
};
}
