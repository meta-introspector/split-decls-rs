macro_rules! deps {
    () => {
        ErrorKind!();
        Err!();
        Many1Count!();
        IResult!();
        Error!();
    };
}

macro_rules! many1_count_test {
    () => {
        deps!();
        # [test] fn many1_count_test () { fn count1_nums (i : & [u8]) -> IResult < & [u8] , usize > { many1_count (pair (digit , tag (","))) . parse (i) } assert_eq ! (count1_nums (& b"123,45,junk" [..]) , Ok ((& b"junk" [..] , 2))) ; assert_eq ! (count1_nums (& b"1,2,3,4,5,6,7,8,9,0,junk" [..]) , Ok ((& b"junk" [..] , 10))) ; assert_eq ! (count1_nums (& b"hello" [..]) , Err (Err :: Error (error_position ! (& b"hello" [..] , ErrorKind :: Many1Count)))) ; }
    };
}

many1_count_test!()