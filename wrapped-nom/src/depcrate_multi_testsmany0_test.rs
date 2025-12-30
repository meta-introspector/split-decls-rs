// Generated macro for many0_test (function)
macro_rules! Depcrate_multi_testsmany0_test {
() => {
// Module: crate::multi::tests
// Provides: {"many0_test"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn many0_test () { fn multi (i : & [u8]) -> IResult < & [u8] , Vec < & [u8] > > { many0 (tag ("abcd")) . parse (i) } fn multi_empty (i : & [u8]) -> IResult < & [u8] , Vec < & [u8] > > { many0 (tag ("")) . parse (i) } assert_eq ! (multi (& b"abcdef" [..]) , Ok ((& b"ef" [..] , vec ! [& b"abcd" [..]]))) ; assert_eq ! (multi (& b"abcdabcdefgh" [..]) , Ok ((& b"efgh" [..] , vec ! [& b"abcd" [..] , & b"abcd" [..]]))) ; assert_eq ! (multi (& b"azerty" [..]) , Ok ((& b"azerty" [..] , Vec :: new ()))) ; assert_eq ! (multi (& b"abcdab" [..]) , Err (Err :: Incomplete (Needed :: new (2)))) ; assert_eq ! (multi (& b"abcd" [..]) , Err (Err :: Incomplete (Needed :: new (4)))) ; assert_eq ! (multi (& b"" [..]) , Err (Err :: Incomplete (Needed :: new (4)))) ; assert_eq ! (multi_empty (& b"abcdef" [..]) , Err (Err :: Error (error_position ! (& b"abcdef" [..] , ErrorKind :: Many0)))) ; }
};
}
