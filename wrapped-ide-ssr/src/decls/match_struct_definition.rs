macro_rules! match_struct_definition {
    () => {
        # [test] fn match_struct_definition () { let code = r#"
        struct Option<T> {}
        struct Bar {}
        struct Foo {name: Option<String>}"# ; assert_matches ("struct $n {$f: Option<String>}" , code , & ["struct Foo {name: Option<String>}"]) ; }
    };
}

match_struct_definition!()