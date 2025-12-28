macro_rules! replace_within_macro_expansion {
    () => {
        # [test] fn replace_within_macro_expansion () { assert_ssr_transform ("$a.foo() ==>> bar($a)" , r#"
            macro_rules! macro1 {
                ($a:expr) => {$a}
            }
            fn bar() {}
            fn f() {macro1!(5.x().foo().o2())}
            "# , expect ! [[r#"
            macro_rules! macro1 {
                ($a:expr) => {$a}
            }
            fn bar() {}
            fn f() {macro1!(bar(5.x()).o2())}
            "#]] ,) }
    };
}

replace_within_macro_expansion!();