macro_rules! deps {
    () => {
        IResult!();
        Error!();
        ErrorKind!();
        Tag!();
        Err!();
    };
}

macro_rules! infinite_many {
    () => {
        deps!();
        # [test] # [cfg (feature = "std")] fn infinite_many () { fn tst (input : & [u8]) -> IResult < & [u8] , & [u8] > { Err (Err :: Error (error_position ! (input , ErrorKind :: Tag))) } fn multi0 (i : & [u8]) -> IResult < & [u8] , Vec < & [u8] > > { many0 (tst) . parse (i) } let a = & b"abcdef" [..] ; assert_eq ! (multi0 (a) , Ok ((a , Vec :: new ()))) ; fn multi1 (i : & [u8]) -> IResult < & [u8] , Vec < & [u8] > > { many1 (tst) . parse (i) } let a = & b"abcdef" [..] ; assert_eq ! (multi1 (a) , Err (Err :: Error (error_position ! (a , ErrorKind :: Tag)))) ; }
    };
}

infinite_many!();