macro_rules! deps {
    () => {
        Error!();
        ErrorKind!();
        Tag!();
        IResult!();
        Needed!();
        Err!();
    };
}

macro_rules! length_count_test {
    () => {
        deps!();
        # [test] # [cfg (feature = "alloc")] fn length_count_test () { fn cnt (i : & [u8]) -> IResult < & [u8] , Vec < & [u8] > > { length_count (number , tag ("abc")) . parse (i) } assert_eq ! (cnt (& b"2abcabcabcdef" [..]) , Ok ((& b"abcdef" [..] , vec ! [& b"abc" [..] , & b"abc" [..]]))) ; assert_eq ! (cnt (& b"2ab" [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (cnt (& b"3abcab" [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (cnt (& b"xxx" [..]) , Err (Err :: Error (error_position ! (& b"xxx" [..] , ErrorKind :: Digit)))) ; assert_eq ! (cnt (& b"2abcxxx" [..]) , Err (Err :: Error (error_position ! (& b"xxx" [..] , ErrorKind :: Tag)))) ; }
    };
}

length_count_test!()