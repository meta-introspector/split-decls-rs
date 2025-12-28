macro_rules! deps {
    () => {
        IResult!();
    };
}

macro_rules! alt_array {
    () => {
        deps!();
        # [test] fn alt_array () { fn alt1 (i : & [u8]) -> IResult < & [u8] , & [u8] > { alt ([tag ("a") , tag ("bc") , tag ("def")]) . parse (i) } let a = & b"a" [..] ; assert_eq ! (alt1 (a) , Ok ((& b"" [..] , (& b"a" [..])))) ; let bc = & b"bc" [..] ; assert_eq ! (alt1 (bc) , Ok ((& b"" [..] , (& b"bc" [..])))) ; let defg = & b"defg" [..] ; assert_eq ! (alt1 (defg) , Ok ((& b"g" [..] , (& b"def" [..])))) ; }
    };
}

alt_array!()