// Generated macro for test_edition_handling_out (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_edition_handling_out {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_edition_handling_out"}
// Dependencies: {}
# [test] fn test_edition_handling_out () { check (r#"
//- /main.rs crate:main deps:old edition:2021
macro_rules! r#try {
    ($it:expr) => {
        $it?
    };
}
fn f() {
    old::invoke_bare_try!(0);
}
//- /old.rs crate:old edition:2015
#[macro_export]
macro_rules! invoke_bare_try {
    ($it:expr) => {
        try!($it)
    };
}
 "# , expect ! [[r#"
macro_rules! r#try {
    ($it:expr) => {
        $it?
    };
}
fn f() {
    try!(0);
}
"#]] ,) ; }
};
}
