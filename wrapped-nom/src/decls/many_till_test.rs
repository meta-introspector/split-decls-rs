macro_rules! deps {
    () => {
        IResult!();
        Err!();
        Tag!();
        Error!();
        ErrorKind!();
        ManyTill!();
    };
}

macro_rules! many_till_test {
    () => {
        deps!();
        # [test] # [cfg (feature = "alloc")] fn many_till_test () { # [allow (clippy :: type_complexity)] fn multi (i : & [u8]) -> IResult < & [u8] , (Vec < & [u8] > , & [u8]) > { many_till (tag ("abcd") , tag ("efgh")) . parse (i) } let a = b"abcdabcdefghabcd" ; let b = b"efghabcd" ; let c = b"azerty" ; let res_a = (vec ! [& b"abcd" [..] , & b"abcd" [..]] , & b"efgh" [..]) ; let res_b : (Vec < & [u8] > , & [u8]) = (Vec :: new () , & b"efgh" [..]) ; assert_eq ! (multi (& a [..]) , Ok ((& b"abcd" [..] , res_a))) ; assert_eq ! (multi (& b [..]) , Ok ((& b"abcd" [..] , res_b))) ; assert_eq ! (multi (& c [..]) , Err (Err :: Error (error_node_position ! (& c [..] , ErrorKind :: ManyTill , error_position ! (& c [..] , ErrorKind :: Tag))))) ; }
    };
}

many_till_test!();