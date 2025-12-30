// Generated macro for terminated_test (function)
macro_rules! Depcrate_sequence_teststerminated_test {
() => {
// Module: crate::sequence::tests
// Provides: {"terminated_test"}
// Dependencies: {}
# [test] fn terminated_test () { fn terminated_abcd_efgh (i : & [u8]) -> IResult < & [u8] , & [u8] > { terminated (tag ("abcd") , tag ("efgh")) . parse (i) } assert_eq ! (terminated_abcd_efgh (& b"abcdefghijkl" [..]) , Ok ((& b"ijkl" [..] , & b"abcd" [..]))) ; assert_eq ! (terminated_abcd_efgh (& b"ab" [..]) , Err (Err :: Incomplete (Needed :: new (2)))) ; assert_eq ! (terminated_abcd_efgh (& b"abcde" [..]) , Err (Err :: Incomplete (Needed :: new (3)))) ; assert_eq ! (terminated_abcd_efgh (& b"xxx" [..]) , Err (Err :: Error (error_position ! (& b"xxx" [..] , ErrorKind :: Tag)))) ; assert_eq ! (terminated_abcd_efgh (& b"xxxxdef" [..]) , Err (Err :: Error (error_position ! (& b"xxxxdef" [..] , ErrorKind :: Tag)))) ; assert_eq ! (terminated_abcd_efgh (& b"abcdxxxx" [..]) , Err (Err :: Error (error_position ! (& b"xxxx" [..] , ErrorKind :: Tag)))) ; }
};
}
