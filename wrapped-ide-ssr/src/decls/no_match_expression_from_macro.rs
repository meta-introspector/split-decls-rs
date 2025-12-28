macro_rules! no_match_expression_from_macro {
    () => {
        # [test] fn no_match_expression_from_macro () { assert_no_match ("$a.clone()" , r#"
            macro_rules! m1 {
                () => {42.clone()}
            }
            fn f1() {m1!()}
            "# ,) ; }
    };
}

no_match_expression_from_macro!();