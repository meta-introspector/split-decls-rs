macro_rules! test_default_expand_with_cfg {
    () => {
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

test_default_expand_with_cfg!()