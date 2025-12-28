macro_rules! ssr_struct_def {
    () => {
        # [test] fn ssr_struct_def () { assert_ssr_transform ("struct Foo { $f: $t } ==>> struct Foo($t);" , r#"struct Foo { field: i32 }"# , expect ! [[r#"struct Foo(i32);"#]] ,) }
    };
}

ssr_struct_def!()