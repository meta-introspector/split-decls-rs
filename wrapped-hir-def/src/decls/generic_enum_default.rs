macro_rules! generic_enum_default {
    () => {
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

generic_enum_default!()