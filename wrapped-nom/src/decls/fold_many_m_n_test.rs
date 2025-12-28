macro_rules! deps {
    () => {
        Needed!();
        Tag!();
        Error!();
        ErrorKind!();
        Err!();
        IResult!();
    };
}

macro_rules! fold_many_m_n_test {
    () => {
        deps!();
        # [test] # [cfg (feature = "alloc")] fn fold_many_m_n_test () { fn fold_into_vec < T > (mut acc : Vec < T > , item : T) -> Vec < T > { acc . push (item) ; acc } fn multi (i : & [u8]) -> IResult < & [u8] , Vec < & [u8] > > { fold_many_m_n (2 , 4 , tag ("Abcd") , Vec :: new , fold_into_vec) . parse (i) } let a = & b"Abcdef" [..] ; let b = & b"AbcdAbcdefgh" [..] ; let c = & b"AbcdAbcdAbcdAbcdefgh" [..] ; let d = & b"AbcdAbcdAbcdAbcdAbcdefgh" [..] ; let e = & b"AbcdAb" [..] ; assert_eq ! (multi (a) , Err (Err :: Error (error_position ! (& b"ef" [..] , ErrorKind :: Tag)))) ; let res1 = vec ! [& b"Abcd" [..] , & b"Abcd" [..]] ; assert_eq ! (multi (b) , Ok ((& b"efgh" [..] , res1))) ; let res2 = vec ! [& b"Abcd" [..] , & b"Abcd" [..] , & b"Abcd" [..] , & b"Abcd" [..]] ; assert_eq ! (multi (c) , Ok ((& b"efgh" [..] , res2))) ; let res3 = vec ! [& b"Abcd" [..] , & b"Abcd" [..] , & b"Abcd" [..] , & b"Abcd" [..]] ; assert_eq ! (multi (d) , Ok ((& b"Abcdefgh" [..] , res3))) ; assert_eq ! (multi (e) , Err (Err :: Incomplete (Needed :: new (2)))) ; }
    };
}

fold_many_m_n_test!();