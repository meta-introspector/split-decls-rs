// Generated macro for preceded_test (function)
macro_rules! Depcrate_sequence_testspreceded_test {
() => {
// Module: crate::sequence::tests
// Provides: {"preceded_test"}
// Dependencies: {}
# [test] fn preceded_test () { fn preceded_abcd_efgh (i : & [u8]) -> IResult < & [u8] , & [u8] > { preceded (tag ("abcd") , tag ("efgh")) . parse (i) } assert_eq ! (preceded_abcd_efgh (& b"abcdefghijkl" [..]) , Ok ((& b"ijkl" [..] , & b"efgh" [..]))) ; assert_eq ! (preceded_abcd_efgh (& b"ab" [..]) , Err (Err :: Incomplete (Needed :: new (2)))) ; assert_eq ! (preceded_abcd_efgh (& b"abcde" [..]) , Err (Err :: Incomplete (Needed :: new (3)))) ; assert_eq ! (preceded_abcd_efgh (& b"xxx" [..]) , Err (Err :: Error (error_position ! (& b"xxx" [..] , ErrorKind :: Tag)))) ; assert_eq ! (preceded_abcd_efgh (& b"xxxxdef" [..]) , Err (Err :: Error (error_position ! (& b"xxxxdef" [..] , ErrorKind :: Tag)))) ; assert_eq ! (preceded_abcd_efgh (& b"abcdxxx" [..]) , Err (Err :: Error (error_position ! (& b"xxx" [..] , ErrorKind :: Tag)))) ; }
};
}
