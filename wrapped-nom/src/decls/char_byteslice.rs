macro_rules! deps {
    () => {
        IResult!();
        Err!();
        ErrorKind!();
        Char!();
        Error!();
    };
}

macro_rules! char_byteslice {
    () => {
        deps!();
        # [test] fn char_byteslice () { fn f (i : & [u8]) -> IResult < & [u8] , char > { char ('c') (i) } let a = & b"abcd" [..] ; assert_eq ! (f (a) , Err (Err :: Error (error_position ! (a , ErrorKind :: Char)))) ; let b = & b"cde" [..] ; assert_eq ! (f (b) , Ok ((& b"de" [..] , 'c'))) ; }
    };
}

char_byteslice!();