macro_rules! match_pattern {
    () => {
        # [test] fn match_pattern () { assert_matches ("Some($a)" , "struct Some(); fn f() {if let Some(x) = foo() {}}" , & ["Some(x)"]) ; }
    };
}

match_pattern!();