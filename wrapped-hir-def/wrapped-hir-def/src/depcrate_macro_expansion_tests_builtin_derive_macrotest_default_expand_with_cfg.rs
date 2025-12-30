// Generated macro for test_default_expand_with_cfg (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_derive_macrotest_default_expand_with_cfg {
() => {
// Module: crate::macro_expansion_tests::builtin_derive_macro
// Provides: {"test_default_expand_with_cfg"}
// Dependencies: {}
# [test] fn test_default_expand_with_cfg () { check (r#"
//- minicore: derive, default
#[derive(Default)]
struct Foo {
    field1: i32,
    #[cfg(never)]
    field2: (),
    #[cfg(feature = "never")]
    field3: (),
    #[cfg(not(feature = "never"))]
    field4: (),
}
#[derive(Default)]
enum Bar {
    Foo,
    #[cfg_attr(not(never), default)]
    Bar,
}
"# , expect ! [[r##"
#[derive(Default)]
struct Foo {
    field1: i32,
    #[cfg(never)]
    field2: (),
    #[cfg(feature = "never")]
    field3: (),
    #[cfg(not(feature = "never"))]
    field4: (),
}
#[derive(Default)]
enum Bar {
    Foo,
    #[cfg_attr(not(never), default)]
    Bar,
}

impl <> $crate::default::Default for Foo< > where {
    fn default() -> Self {
        Foo {
            field1: $crate::default::Default::default(), field4: $crate::default::Default::default(),
        }
    }
}
impl <> $crate::default::Default for Bar< > where {
    fn default() -> Self {
        Bar::Bar
    }
}"##]] ,) ; }
};
}
