macro_rules! add_parenthesis_when_necessary {
    () => {
        # [test] fn add_parenthesis_when_necessary () { assert_ssr_transform ("foo($a) ==>> $a.to_string()" , r#"
        fn foo(_: i32) {}
        fn bar3(v: i32) {
            foo(1 + 2);
            foo(-v);
        }
        "# , expect ! [[r#"
            fn foo(_: i32) {}
            fn bar3(v: i32) {
                (1 + 2).to_string();
                (-v).to_string();
            }
        "#]] ,) }
    };
}

add_parenthesis_when_necessary!();