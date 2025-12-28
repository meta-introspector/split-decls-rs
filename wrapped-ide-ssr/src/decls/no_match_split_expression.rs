macro_rules! no_match_split_expression {
    () => {
        # [test] fn no_match_split_expression () { assert_no_match ("$a.clone()" , r#"
            macro_rules! m1 {
                ($x:expr) => {$x.clone()}
            }
            fn f1() {m1!(42)}
            "# ,) ; }
    };
}

no_match_split_expression!()