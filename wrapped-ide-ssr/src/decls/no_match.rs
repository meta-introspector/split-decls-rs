macro_rules! no_match {
    () => {
        # [test] fn no_match () { assert_no_match ("1 + 3" , "fn f() -> i32 {1  +  2}") ; }
    };
}

no_match!()