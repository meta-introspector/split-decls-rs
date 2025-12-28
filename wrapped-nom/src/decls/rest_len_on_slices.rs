macro_rules! rest_len_on_slices {
    () => {
        # [test] fn rest_len_on_slices () { let input : & [u8] = & b"Hello, world!" [..] ; assert_parse ! (rest_len (input) , Ok ((input , input . len ()))) ; }
    };
}

rest_len_on_slices!();