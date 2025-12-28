macro_rules! default_enum_without_default_attr {
    () => {
        # [test] fn default_enum_without_default_attr () { check_errors (r#"
//- minicore: default, derive

#[derive(Default)]
enum Foo {
    Bar,
}
    "# , expect ! ["1..41: `#[derive(Default)]` on enum with no `#[default]`"] ,) ; }
    };
}

default_enum_without_default_attr!()