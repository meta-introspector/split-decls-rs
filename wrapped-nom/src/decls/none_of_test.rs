macro_rules! deps {
    () => {
        IResult!();
        Err!();
        ErrorKind!();
        Error!();
    };
}

macro_rules! none_of_test {
    () => {
        deps!();
        # [test] fn none_of_test () { fn f (i : & [u8]) -> IResult < & [u8] , char > { none_of ("ab") (i) } let a = & b"abcd" [..] ; assert_eq ! (f (a) , Err (Err :: Error (error_position ! (a , ErrorKind :: NoneOf)))) ; let b = & b"cde" [..] ; assert_eq ! (f (b) , Ok ((& b"de" [..] , 'c'))) ; }
    };
}

none_of_test!();