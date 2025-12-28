macro_rules! match_failure_reasons {
    () => {
        # [test] fn match_failure_reasons () { let code = r#"
        fn bar() {}
        macro_rules! foo {
            ($a:expr) => {
                1 + $a + 2
            };
        }
        fn f1() {
            bar(1, 2);
            foo!(5 + 43.to_string() + 5);
        }
        "# ; assert_match_failure_reason ("bar($a, 3)" , code , "bar(1, 2)" , r#"Pattern wanted token '3' (INT_NUMBER), but code had token '2' (INT_NUMBER)"# ,) ; assert_match_failure_reason ("42.to_string()" , code , "43.to_string()" , r#"Pattern wanted token '42' (INT_NUMBER), but code had token '43' (INT_NUMBER)"# ,) ; }
    };
}

match_failure_reasons!();