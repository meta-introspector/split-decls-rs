macro_rules! literal_constraint {
    () => {
        # [test] fn literal_constraint () { cov_mark :: check ! (literal_constraint) ; let code = r#"
        enum Option<T> { Some(T), None }
        use Option::Some;
        fn f1() {
            let x1 = Some(42);
            let x2 = Some("foo");
            let x3 = Some(x1);
            let x4 = Some(40 + 2);
            let x5 = Some(true);
        }
        "# ; assert_matches ("Some(${a:kind(literal)})" , code , & ["Some(42)" , "Some(\"foo\")" , "Some(true)"]) ; assert_matches ("Some(${a:not(kind(literal))})" , code , & ["Some(x1)" , "Some(40 + 2)"]) ; }
    };
}

literal_constraint!();