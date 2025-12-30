// Generated macro for delimited_test (function)
macro_rules! Depcrate_sequence_testsdelimited_test {
() => {
// Module: crate::sequence::tests
// Provides: {"delimited_test"}
// Dependencies: {}
# [test] fn delimited_test () { fn delimited_abc_def_ghi (i : & [u8]) -> IResult < & [u8] , & [u8] > { delimited (tag ("abc") , tag ("def") , tag ("ghi")) . parse (i) } assert_eq ! (delimited_abc_def_ghi (& b"abcdefghijkl" [..]) , Ok ((& b"jkl" [..] , & b"def" [..]))) ; assert_eq ! (delimited_abc_def_ghi (& b"ab" [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (delimited_abc_def_ghi (& b"abcde" [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (delimited_abc_def_ghi (& b"abcdefgh" [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (delimited_abc_def_ghi (& b"xxx" [..]) , Err (Err :: Error (error_position ! (& b"xxx" [..] , ErrorKind :: Tag)))) ; assert_eq ! (delimited_abc_def_ghi (& b"xxxdefghi" [..]) , Err (Err :: Error (error_position ! (& b"xxxdefghi" [..] , ErrorKind :: Tag) ,))) ; assert_eq ! (delimited_abc_def_ghi (& b"abcxxxghi" [..]) , Err (Err :: Error (error_position ! (& b"xxxghi" [..] , ErrorKind :: Tag)))) ; assert_eq ! (delimited_abc_def_ghi (& b"abcdefxxx" [..]) , Err (Err :: Error (error_position ! (& b"xxx" [..] , ErrorKind :: Tag)))) ; }
};
}
