// Generated macro for test_default_expand (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_derive_macrotest_default_expand {
() => {
// Module: crate::macro_expansion_tests::builtin_derive_macro
// Provides: {"test_default_expand"}
// Dependencies: {}
# [test] fn test_default_expand () { check (r#"
//- minicore: derive, default
#[derive(Default)]
struct Foo {
    field1: i32,
    field2: (),
}
#[derive(Default)]
enum Bar {
    Foo(u8),
    #[default]
    Bar,
}
"# , expect ! [[r#"
#[derive(Default)]
struct Foo {
    field1: i32,
    field2: (),
}
#[derive(Default)]
enum Bar {
    Foo(u8),
    #[default]
    Bar,
}

impl <> $crate::default::Default for Foo< > where {
    fn default() -> Self {
        Foo {
            field1: $crate::default::Default::default(), field2: $crate::default::Default::default(),
        }
    }
}
impl <> $crate::default::Default for Bar< > where {
    fn default() -> Self {
        Bar::Bar
    }
}"#]] ,) ; }
};
}
