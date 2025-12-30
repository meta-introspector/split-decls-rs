// Generated macro for separated_list1_test (function)
macro_rules! Depcrate_multi_testsseparated_list1_test {
() => {
// Module: crate::multi::tests
// Provides: {"separated_list1_test"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn separated_list1_test () { fn multi (i : & [u8]) -> IResult < & [u8] , Vec < & [u8] > > { separated_list1 (tag (",") , tag ("abcd")) . parse (i) } fn multi_longsep (i : & [u8]) -> IResult < & [u8] , Vec < & [u8] > > { separated_list1 (tag ("..") , tag ("abcd")) . parse (i) } fn empty_both (i : & [u8]) -> IResult < & [u8] , Vec < & [u8] > > { separated_list0 (tag ("") , tag ("")) . parse (i) } let a = & b"abcdef" [..] ; let b = & b"abcd,abcdef" [..] ; let c = & b"azerty" [..] ; let d = & b"abcd,abcd,ef" [..] ; let f = & b"abc" [..] ; let g = & b"abcd." [..] ; let h = & b"abcd,abc" [..] ; let res1 = vec ! [& b"abcd" [..]] ; assert_eq ! (multi (a) , Ok ((& b"ef" [..] , res1))) ; let res2 = vec ! [& b"abcd" [..] , & b"abcd" [..]] ; assert_eq ! (multi (b) , Ok ((& b"ef" [..] , res2))) ; assert_eq ! (multi (c) , Err (Err :: Error (error_position ! (c , ErrorKind :: Tag)))) ; assert_eq ! (empty_both (f) , Err (Err :: Error (error_position ! (f , ErrorKind :: SeparatedList)))) ; let res3 = vec ! [& b"abcd" [..] , & b"abcd" [..]] ; assert_eq ! (multi (d) , Ok ((& b",ef" [..] , res3))) ; assert_eq ! (multi (f) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (multi_longsep (g) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (multi (h) , Err (Err :: Incomplete (Needed :: new (1)))) ; }
};
}
