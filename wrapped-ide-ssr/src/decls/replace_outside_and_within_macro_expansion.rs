macro_rules! replace_outside_and_within_macro_expansion {
    () => {
        # [test] fn replace_outside_and_within_macro_expansion () { assert_ssr_transform ("foo($a) ==>> bar($a)" , r#"
            fn foo() {} fn bar() {}
            macro_rules! macro1 {
                ($a:expr) => {$a}
            }
            fn f() {foo(foo(macro1!(foo(foo(42)))))}
            "# , expect ! [[r#"
            fn foo() {} fn bar() {}
            macro_rules! macro1 {
                ($a:expr) => {$a}
            }
            fn f() {bar(bar(macro1!(bar(bar(42)))))}
        "#]] ,) }
    };
}

replace_outside_and_within_macro_expansion!()