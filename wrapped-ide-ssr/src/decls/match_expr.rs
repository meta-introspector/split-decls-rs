macro_rules! match_expr {
    () => {
        # [test] fn match_expr () { let code = r#"
        fn foo() {}
        fn f() -> i32 {foo(40 + 2, 42)}"# ; assert_matches ("foo($a, $b)" , code , & ["foo(40 + 2, 42)"]) ; assert_no_match ("foo($a, $b, $c)" , code) ; assert_no_match ("foo($a)" , code) ; }
    };
}

match_expr!()