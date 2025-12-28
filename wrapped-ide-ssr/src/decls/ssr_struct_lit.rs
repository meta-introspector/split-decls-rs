macro_rules! ssr_struct_lit {
    () => {
        # [test] fn ssr_struct_lit () { assert_ssr_transform ("Foo{a: $a, b: $b} ==>> Foo::new($a, $b)" , r#"
            struct Foo() {}
            impl Foo { fn new() {} }
            fn main() { Foo{b:2, a:1} }
            "# , expect ! [[r#"
            struct Foo() {}
            impl Foo { fn new() {} }
            fn main() { Foo::new(1, 2) }
        "#]] ,) }
    };
}

ssr_struct_lit!()