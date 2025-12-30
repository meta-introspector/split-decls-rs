// Generated macro for pattern_is_a_single_segment_path (function)
macro_rules! Depcrate_testspattern_is_a_single_segment_path {
() => {
// Module: crate::tests
// Provides: {"pattern_is_a_single_segment_path"}
// Dependencies: {}
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
