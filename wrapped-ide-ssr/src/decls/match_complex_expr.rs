macro_rules! match_complex_expr {
    () => {
        # [test] fn match_complex_expr () { let code = r#"
        fn foo() {} fn bar() {}
        fn f() -> i32 {foo(bar(40, 2), 42)}"# ; assert_matches ("foo($a, $b)" , code , & ["foo(bar(40, 2), 42)"]) ; assert_no_match ("foo($a, $b, $c)" , code) ; assert_no_match ("foo($a)" , code) ; assert_matches ("bar($a, $b)" , code , & ["bar(40, 2)"]) ; }
    };
}

match_complex_expr!()