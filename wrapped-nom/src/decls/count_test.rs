macro_rules! deps {
    () => {
        Error!();
        ErrorKind!();
        Tag!();
        IResult!();
        Err!();
        Needed!();
    };
}

macro_rules! count_test {
    () => {
        deps!();
        # [test] # [cfg (feature = "alloc")] fn count_test () { const TIMES : usize = 2 ; fn cnt_2 (i : & [u8]) -> IResult < & [u8] , Vec < & [u8] > > { count (tag ("abc") , TIMES) . parse (i) } assert_eq ! (cnt_2 (& b"abcabcabcdef" [..]) , Ok ((& b"abcdef" [..] , vec ! [& b"abc" [..] , & b"abc" [..]]))) ; assert_eq ! (cnt_2 (& b"ab" [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (cnt_2 (& b"abcab" [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (cnt_2 (& b"xxx" [..]) , Err (Err :: Error (error_position ! (& b"xxx" [..] , ErrorKind :: Tag)))) ; assert_eq ! (cnt_2 (& b"xxxabcabcdef" [..]) , Err (Err :: Error (error_position ! (& b"xxxabcabcdef" [..] , ErrorKind :: Tag)))) ; assert_eq ! (cnt_2 (& b"abcxxxabcdef" [..]) , Err (Err :: Error (error_position ! (& b"xxxabcdef" [..] , ErrorKind :: Tag)))) ; }
    };
}

count_test!();