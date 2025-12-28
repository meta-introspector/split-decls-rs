macro_rules! deps {
    () => {
        IResult!();
        ErrorKind!();
        Error!();
        Err!();
    };
}

macro_rules! one_of_test {
    () => {
        deps!();
        # [test] fn one_of_test () { fn f (i : & [u8]) -> IResult < & [u8] , char > { one_of ("ab") (i) } let a = & b"abcd" [..] ; assert_eq ! (f (a) , Ok ((& b"bcd" [..] , 'a'))) ; let b = & b"cde" [..] ; assert_eq ! (f (b) , Err (Err :: Error (error_position ! (b , ErrorKind :: OneOf)))) ; fn utf8 (i : & str) -> IResult < & str , char > { one_of ("+\u{FF0B}") (i) } assert ! (utf8 ("+") . is_ok ()) ; assert ! (utf8 ("\u{FF0B}") . is_ok ()) ; }
    };
}

one_of_test!()