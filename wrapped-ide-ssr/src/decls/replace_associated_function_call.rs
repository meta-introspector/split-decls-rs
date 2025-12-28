macro_rules! replace_associated_function_call {
    () => {
        # [test] fn replace_associated_function_call () { assert_ssr_transform ("Foo::new() ==>> Bar::new()" , r#"
            struct Foo {}
            impl Foo { fn new() {} }
            struct Bar {}
            impl Bar { fn new() {} }
            fn f1() {Foo::new();}
            "# , expect ! [[r#"
            struct Foo {}
            impl Foo { fn new() {} }
            struct Bar {}
            impl Bar { fn new() {} }
            fn f1() {Bar::new();}
        "#]] ,) ; }
    };
}

replace_associated_function_call!()