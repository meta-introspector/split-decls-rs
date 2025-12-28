macro_rules! replace_nonpath_within_selection {
    () => {
        # [test] fn replace_nonpath_within_selection () { cov_mark :: check ! (replace_nonpath_within_selection) ; assert_ssr_transform ("$a + $b ==>> $b * $a" , r#"
        fn main() {
            let v = 1 + 2;$0
            let v2 = 3 + 3;
            let v3 = 4 + 5;$0
            let v4 = 6 + 7;
        }"# , expect ! [[r#"
            fn main() {
                let v = 1 + 2;
                let v2 = 3 * 3;
                let v3 = 5 * 4;
                let v4 = 6 + 7;
            }"#]] ,) ; }
    };
}

replace_nonpath_within_selection!();