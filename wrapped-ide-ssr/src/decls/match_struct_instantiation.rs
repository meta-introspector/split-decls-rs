macro_rules! match_struct_instantiation {
    () => {
        # [test] fn match_struct_instantiation () { let code = r#"
        struct Foo {bar: i32, baz: i32}
        fn f() {Foo {bar: 1, baz: 2}}"# ; assert_matches ("Foo {bar: 1, baz: 2}" , code , & ["Foo {bar: 1, baz: 2}"]) ; assert_matches ("Foo {$a: $b, $c: $d}" , code , & ["Foo {bar: 1, baz: 2}"]) ; assert_matches ("Foo {}" , "struct Foo {} fn f() {Foo {}}" , & ["Foo {}"]) ; }
    };
}

match_struct_instantiation!();