macro_rules! match_binary_op {
    () => {
        # [test] fn match_binary_op () { assert_matches ("$a + $b" , "fn f() {1 + 2 + 3 + 4}" , & ["1 + 2" , "1 + 2 + 3" , "1 + 2 + 3 + 4"]) ; }
    };
}

match_binary_op!()