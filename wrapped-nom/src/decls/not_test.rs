macro_rules! deps {
    () => {
        IResult!();
        Needed!();
        Not!();
        ErrorKind!();
        Err!();
        Error!();
    };
}

macro_rules! not_test {
    () => {
        deps!();
        # [test] fn not_test () { fn not_aaa (i : & [u8]) -> IResult < & [u8] , () > { not (tag ("aaa")) . parse (i) } assert_eq ! (not_aaa (& b"aaa" [..]) , Err (Err :: Error (error_position ! (& b"aaa" [..] , ErrorKind :: Not)))) ; assert_eq ! (not_aaa (& b"aa" [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (not_aaa (& b"abcd" [..]) , Ok ((& b"abcd" [..] , ()))) ; }
    };
}

not_test!()