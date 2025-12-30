// Generated macro for mbe_are_not_attributes (function)
macro_rules! Depcrate_macro_expansion_tests_mbembe_are_not_attributes {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"mbe_are_not_attributes"}
// Dependencies: {}
# [test] fn mbe_are_not_attributes () { check (r#"
macro_rules! error {
    () => {struct Bar}
}

#[error]
struct Foo;
"# , expect ! [[r##"
macro_rules! error {
    () => {struct Bar}
}

#[error]
struct Foo;
"##]] ,) }
};
}
