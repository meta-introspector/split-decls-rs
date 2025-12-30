// Generated macro for default_enum_without_default_attr (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_derive_macrodefault_enum_without_default_attr {
() => {
// Module: crate::macro_expansion_tests::builtin_derive_macro
// Provides: {"default_enum_without_default_attr"}
// Dependencies: {}
# [test] fn default_enum_without_default_attr () { check_errors (r#"
//- minicore: default, derive

#[derive(Default)]
enum Foo {
    Bar,
}
    "# , expect ! ["1..41: `#[derive(Default)]` on enum with no `#[default]`"] ,) ; }
};
}
