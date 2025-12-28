macro_rules! rest_on_slices {
    () => {
        # [test] fn rest_on_slices () { let input : & [u8] = & b"Hello, world!" [..] ; let empty : & [u8] = & b"" [..] ; assert_parse ! (rest (input) , Ok ((empty , input))) ; }
    };
}

rest_on_slices!();