macro_rules! match_fn_definition {
    () => {
        # [test] fn match_fn_definition () { assert_matches ("fn $a($b: $t) {$c}" , "fn f(a: i32) {bar()}" , & ["fn f(a: i32) {bar()}"]) ; }
    };
}

match_fn_definition!()