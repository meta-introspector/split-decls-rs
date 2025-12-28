macro_rules! ssr_nested_function {
    () => {
        # [test] fn ssr_nested_function () { assert_ssr_transform ("foo($a, $b, $c) ==>> bar($c, baz($a, $b))" , r#"
            //- /lib.rs crate:foo
            fn foo() {}
            fn bar() {}
            fn baz() {}
            fn main { foo  (x + value.method(b), x+y-z, true && false) }
            "# , expect ! [[r#"
            fn foo() {}
            fn bar() {}
            fn baz() {}
            fn main { bar(true && false, baz(x + value.method(b), x+y-z)) }
        "#]] ,) }
    };
}

ssr_nested_function!();