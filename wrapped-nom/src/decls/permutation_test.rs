macro_rules! deps {
    () => {
        Err!();
        Permutation!();
        Tag!();
        IResult!();
        Needed!();
        ErrorKind!();
        Error!();
    };
}

macro_rules! permutation_test {
    () => {
        deps!();
        # [test] fn permutation_test () { # [allow (clippy :: type_complexity)] fn perm (i : & [u8]) -> IResult < & [u8] , (& [u8] , & [u8] , & [u8]) > { permutation ((tag ("abcd") , tag ("efg") , tag ("hi"))) . parse (i) } let expected = (& b"abcd" [..] , & b"efg" [..] , & b"hi" [..]) ; let a = & b"abcdefghijk" [..] ; assert_eq ! (perm (a) , Ok ((& b"jk" [..] , expected))) ; let b = & b"efgabcdhijk" [..] ; assert_eq ! (perm (b) , Ok ((& b"jk" [..] , expected))) ; let c = & b"hiefgabcdjk" [..] ; assert_eq ! (perm (c) , Ok ((& b"jk" [..] , expected))) ; let d = & b"efgxyzabcdefghi" [..] ; assert_eq ! (perm (d) , Err (Err :: Error (error_node_position ! (& b"efgxyzabcdefghi" [..] , ErrorKind :: Permutation , error_position ! (& b"xyzabcdefghi" [..] , ErrorKind :: Tag))))) ; let e = & b"efgabc" [..] ; assert_eq ! (perm (e) , Err (Err :: Incomplete (Needed :: new (1)))) ; }
    };
}

permutation_test!()