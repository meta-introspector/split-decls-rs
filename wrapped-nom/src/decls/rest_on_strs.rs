macro_rules! rest_on_strs {
    () => {
        # [test] fn rest_on_strs () { let input : & str = "Hello, world!" ; let empty : & str = "" ; assert_parse ! (rest (input) , Ok ((empty , input))) ; }
    };
}

rest_on_strs!()