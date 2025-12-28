macro_rules! preserves_whitespace_within_macro_expansion {
    () => {
        # [test] fn preserves_whitespace_within_macro_expansion () { assert_ssr_transform ("$a + $b ==>> $b - $a" , r#"
            macro_rules! macro1 {
                ($a:expr) => {$a}
            }
            fn f() {macro1!(1   *   2 + 3 + 4)}
            "# , expect ! [[r#"
            macro_rules! macro1 {
                ($a:expr) => {$a}
            }
            fn f() {macro1!(4 - (3 - 1   *   2))}
            "#]] ,) }
    };
}

preserves_whitespace_within_macro_expansion!()