// Generated macro for pair_test (function)
macro_rules! Depcrate_sequence_testspair_test {
() => {
// Module: crate::sequence::tests
// Provides: {"pair_test"}
// Dependencies: {}
# [test] fn pair_test () { fn pair_abc_def (i : & [u8]) -> IResult < & [u8] , (& [u8] , & [u8]) > { pair (tag ("abc") , tag ("def")) . parse (i) } assert_eq ! (pair_abc_def (& b"abcdefghijkl" [..]) , Ok ((& b"ghijkl" [..] , (& b"abc" [..] , & b"def" [..])))) ; assert_eq ! (pair_abc_def (& b"ab" [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (pair_abc_def (& b"abcd" [..]) , Err (Err :: Incomplete (Needed :: new (2)))) ; assert_eq ! (pair_abc_def (& b"xxx" [..]) , Err (Err :: Error (error_position ! (& b"xxx" [..] , ErrorKind :: Tag)))) ; assert_eq ! (pair_abc_def (& b"xxxdef" [..]) , Err (Err :: Error (error_position ! (& b"xxxdef" [..] , ErrorKind :: Tag)))) ; assert_eq ! (pair_abc_def (& b"abcxxx" [..]) , Err (Err :: Error (error_position ! (& b"xxx" [..] , ErrorKind :: Tag)))) ; }
};
}
