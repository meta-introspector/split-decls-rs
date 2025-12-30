// Generated macro for generic_enum_default (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_derive_macrogeneric_enum_default {
() => {
// Module: crate::macro_expansion_tests::builtin_derive_macro
// Provides: {"generic_enum_default"}
// Dependencies: {}
# [test] fn generic_enum_default () { check (r#"
//- minicore: default, derive

#[derive(Default)]
enum Foo<T> {
    Bar(T),
    #[default]
    Baz,
}
"# , expect ! [[r#"

#[derive(Default)]
enum Foo<T> {
    Bar(T),
    #[default]
    Baz,
}

impl <T, > $crate::default::Default for Foo<T, > where {
    fn default() -> Self {
        Foo::Baz
    }
}"#]] ,) ; }
};
}
