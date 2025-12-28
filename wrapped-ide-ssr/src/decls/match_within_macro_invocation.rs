macro_rules! match_within_macro_invocation {
    () => {
        # [test] fn match_within_macro_invocation () { let code = r#"
            macro_rules! foo {
                ($a:stmt; $b:expr) => {
                    $b
                };
            }
            struct A {}
            impl A {
                fn bar() {}
            }
            fn f1() {
                let aaa = A {};
                foo!(macro_ignores_this(); aaa.bar());
            }
        "# ; assert_matches ("$a.bar()" , code , & ["aaa.bar()"]) ; }
    };
}

match_within_macro_invocation!();