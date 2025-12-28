macro_rules! ignores_whitespace {
    () => {
        # [test] fn ignores_whitespace () { assert_matches ("1+2" , "fn f() -> i32 {1  +  2}" , & ["1  +  2"]) ; assert_matches ("1 + 2" , "fn f() -> i32 {1+2}" , & ["1+2"]) ; }
    };
}

ignores_whitespace!();