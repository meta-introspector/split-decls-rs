macro_rules! replace_binary_op {
    () => {
        # [test] fn replace_binary_op () { assert_ssr_transform ("$a + $b ==>> $b + $a" , "fn f() {2 * 3 + 4 * 5}" , expect ! [["fn f() {4 * 5 + 2 * 3}"]] ,) ; assert_ssr_transform ("$a + $b ==>> $b + $a" , "fn f() {1 + 2 + 3 + 4}" , expect ! [[r#"fn f() {4 + (3 + (2 + 1))}"#]] ,) ; }
    };
}

replace_binary_op!();