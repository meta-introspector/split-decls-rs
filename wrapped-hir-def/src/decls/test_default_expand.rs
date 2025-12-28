macro_rules! test_default_expand {
    () => {
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

test_default_expand!()