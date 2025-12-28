macro_rules! pattern_is_a_single_segment_path {
    () => {
        # [test] fn pattern_is_a_single_segment_path () { cov_mark :: check ! (pattern_is_a_single_segment_path) ; assert_ssr_transform ("foo ==>> bar" , r#"
        fn f1() -> i32 {
            let foo = 1;
            let bar = 2;
            foo
        }
        fn f1() -> i32 {
            let foo = 1;
            let bar = 2;
            foo$0
        }
        "# , expect ! [[r#"
            fn f1() -> i32 {
                let foo = 1;
                let bar = 2;
                foo
            }
            fn f1() -> i32 {
                let foo = 1;
                let bar = 2;
                bar
            }
        "#]] ,) ; }
    };
}

pattern_is_a_single_segment_path!()