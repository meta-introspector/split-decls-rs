macro_rules! multiple_rules_with_nested_matches {
    () => {
        # [test] fn multiple_rules_with_nested_matches () { assert_ssr_transforms (& ["foo1($a) ==>> bar1($a)" , "foo2($a) ==>> bar2($a)"] , r#"
            fn foo1() {} fn foo2() {} fn bar1() {} fn bar2() {}
            fn f() {foo1(foo2(foo1(foo2(foo1(42)))))}
            "# , expect ! [[r#"
            fn foo1() {} fn foo2() {} fn bar1() {} fn bar2() {}
            fn f() {bar1(bar2(bar1(bar2(bar1(42)))))}
        "#]] ,) }
    };
}

multiple_rules_with_nested_matches!()