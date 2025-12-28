macro_rules! replace_self {
    () => {
        # [test] fn replace_self () { assert_ssr_transform ("foo(self) ==>> bar(self)" , r#"
        struct S1 {}
        fn foo(_: &S1) {}
        fn bar(_: &S1) {}
        impl S1 {
            fn f1(&self) {
                foo(self)$0
            }
            fn f2(&self) {
                foo(self)
            }
        }
        "# , expect ! [[r#"
            struct S1 {}
            fn foo(_: &S1) {}
            fn bar(_: &S1) {}
            impl S1 {
                fn f1(&self) {
                    bar(self)
                }
                fn f2(&self) {
                    foo(self)
                }
            }
        "#]] ,) ; }
    };
}

replace_self!()