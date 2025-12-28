macro_rules! deps {
    () => {
        ErrorKind!();
        IResult!();
        Tag!();
        Error!();
        Err!();
        Needed!();
    };
}

macro_rules! alt_incomplete {
    () => {
        deps!();
        # [test] fn alt_incomplete () { fn alt1 (i : & [u8]) -> IResult < & [u8] , & [u8] > { alt ((tag ("a") , tag ("bc") , tag ("def"))) . parse (i) } let a = & b"" [..] ; assert_eq ! (alt1 (a) , Err (Err :: Incomplete (Needed :: new (1)))) ; let a = & b"b" [..] ; assert_eq ! (alt1 (a) , Err (Err :: Incomplete (Needed :: new (1)))) ; let a = & b"bcd" [..] ; assert_eq ! (alt1 (a) , Ok ((& b"d" [..] , & b"bc" [..]))) ; let a = & b"cde" [..] ; assert_eq ! (alt1 (a) , Err (Err :: Error (error_position ! (a , ErrorKind :: Tag)))) ; let a = & b"de" [..] ; assert_eq ! (alt1 (a) , Err (Err :: Incomplete (Needed :: new (1)))) ; let a = & b"defg" [..] ; assert_eq ! (alt1 (a) , Ok ((& b"g" [..] , & b"def" [..]))) ; }
    };
}

alt_incomplete!();