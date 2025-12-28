macro_rules! deps {
    () => {
        Error!();
        Many1!();
        IResult!();
        Err!();
        Needed!();
        ErrorKind!();
    };
}

macro_rules! fold_many1_test {
    () => {
        deps!();
        # [test] # [cfg (feature = "alloc")] fn fold_many1_test () { fn fold_into_vec < T > (mut acc : Vec < T > , item : T) -> Vec < T > { acc . push (item) ; acc } fn multi (i : & [u8]) -> IResult < & [u8] , Vec < & [u8] > > { fold_many1 (tag ("abcd") , Vec :: new , fold_into_vec) . parse (i) } let a = & b"abcdef" [..] ; let b = & b"abcdabcdefgh" [..] ; let c = & b"azerty" [..] ; let d = & b"abcdab" [..] ; let res1 = vec ! [& b"abcd" [..]] ; assert_eq ! (multi (a) , Ok ((& b"ef" [..] , res1))) ; let res2 = vec ! [& b"abcd" [..] , & b"abcd" [..]] ; assert_eq ! (multi (b) , Ok ((& b"efgh" [..] , res2))) ; assert_eq ! (multi (c) , Err (Err :: Error (error_position ! (c , ErrorKind :: Many1)))) ; assert_eq ! (multi (d) , Err (Err :: Incomplete (Needed :: new (2)))) ; }
    };
}

fold_many1_test!()