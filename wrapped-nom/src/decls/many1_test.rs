macro_rules! deps {
    () => {
        IResult!();
        ErrorKind!();
        Error!();
        Needed!();
        Err!();
        Tag!();
    };
}

macro_rules! many1_test {
    () => {
        deps!();
        # [test] # [cfg (feature = "alloc")] fn many1_test () { fn multi (i : & [u8]) -> IResult < & [u8] , Vec < & [u8] > > { many1 (tag ("abcd")) . parse (i) } let a = & b"abcdef" [..] ; let b = & b"abcdabcdefgh" [..] ; let c = & b"azerty" [..] ; let d = & b"abcdab" [..] ; let res1 = vec ! [& b"abcd" [..]] ; assert_eq ! (multi (a) , Ok ((& b"ef" [..] , res1))) ; let res2 = vec ! [& b"abcd" [..] , & b"abcd" [..]] ; assert_eq ! (multi (b) , Ok ((& b"efgh" [..] , res2))) ; assert_eq ! (multi (c) , Err (Err :: Error (error_position ! (c , ErrorKind :: Tag)))) ; assert_eq ! (multi (d) , Err (Err :: Incomplete (Needed :: new (2)))) ; }
    };
}

many1_test!();