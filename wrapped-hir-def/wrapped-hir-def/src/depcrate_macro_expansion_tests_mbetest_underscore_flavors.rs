// Generated macro for test_underscore_flavors (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_underscore_flavors {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_underscore_flavors"}
// Dependencies: {}
# [test] fn test_underscore_flavors () { check (r#"
macro_rules! m1 { ($a:ty) => { ok!(); } }
m1![_];

macro_rules! m2 { ($a:lifetime) => { ok!(); } }
m2!['_];
"# , expect ! [[r#"
macro_rules! m1 { ($a:ty) => { ok!(); } }
ok!();

macro_rules! m2 { ($a:lifetime) => { ok!(); } }
ok!();
"#]] ,) ; }
};
}
