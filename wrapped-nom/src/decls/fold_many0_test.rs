macro_rules! deps {
    () => {
        Error!();
        IResult!();
        Many0!();
        ErrorKind!();
        Needed!();
        Err!();
    };
}

macro_rules! fold_many0_test {
    () => {
        deps!();
        # [test] # [cfg (feature = "alloc")] fn fold_many0_test () { fn fold_into_vec < T > (mut acc : Vec < T > , item : T) -> Vec < T > { acc . push (item) ; acc } fn multi (i : & [u8]) -> IResult < & [u8] , Vec < & [u8] > > { fold_many0 (tag ("abcd") , Vec :: new , fold_into_vec) . parse (i) } fn multi_empty (i : & [u8]) -> IResult < & [u8] , Vec < & [u8] > > { fold_many0 (tag ("") , Vec :: new , fold_into_vec) . parse (i) } assert_eq ! (multi (& b"abcdef" [..]) , Ok ((& b"ef" [..] , vec ! [& b"abcd" [..]]))) ; assert_eq ! (multi (& b"abcdabcdefgh" [..]) , Ok ((& b"efgh" [..] , vec ! [& b"abcd" [..] , & b"abcd" [..]]))) ; assert_eq ! (multi (& b"azerty" [..]) , Ok ((& b"azerty" [..] , Vec :: new ()))) ; assert_eq ! (multi (& b"abcdab" [..]) , Err (Err :: Incomplete (Needed :: new (2)))) ; assert_eq ! (multi (& b"abcd" [..]) , Err (Err :: Incomplete (Needed :: new (4)))) ; assert_eq ! (multi (& b"" [..]) , Err (Err :: Incomplete (Needed :: new (4)))) ; assert_eq ! (multi_empty (& b"abcdef" [..]) , Err (Err :: Error (error_position ! (& b"abcdef" [..] , ErrorKind :: Many0)))) ; }
    };
}

fold_many0_test!()