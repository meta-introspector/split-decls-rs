macro_rules! deps {
    () => {
        Err!();
        Char!();
        IResult!();
        Error!();
        ErrorKind!();
    };
}

macro_rules! char_str {
    () => {
        deps!();
        # [test] fn char_str () { fn f (i : & str) -> IResult < & str , char > { char ('c') (i) } let a = "abcd" ; assert_eq ! (f (a) , Err (Err :: Error (error_position ! (a , ErrorKind :: Char)))) ; let b = "cde" ; assert_eq ! (f (b) , Ok (("de" , 'c'))) ; }
    };
}

char_str!()