macro_rules! deps {
    () => {
        Err!();
        Error!();
        Needed!();
        ErrorKind!();
        IResult!();
    };
}

macro_rules! length_data_test {
    () => {
        deps!();
        # [test] fn length_data_test () { fn take (i : & [u8]) -> IResult < & [u8] , & [u8] > { length_data (number) . parse (i) } assert_eq ! (take (& b"6abcabcabcdef" [..]) , Ok ((& b"abcdef" [..] , & b"abcabc" [..]))) ; assert_eq ! (take (& b"3ab" [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (take (& b"xxx" [..]) , Err (Err :: Error (error_position ! (& b"xxx" [..] , ErrorKind :: Digit)))) ; assert_eq ! (take (& b"2abcxxx" [..]) , Ok ((& b"cxxx" [..] , & b"ab" [..]))) ; }
    };
}

length_data_test!();