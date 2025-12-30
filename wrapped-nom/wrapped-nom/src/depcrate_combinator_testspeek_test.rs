// Generated macro for peek_test (function)
macro_rules! Depcrate_combinator_testspeek_test {
() => {
// Module: crate::combinator::tests
// Provides: {"peek_test"}
// Dependencies: {}
# [test] fn peek_test () { fn peek_tag (i : & [u8]) -> IResult < & [u8] , & [u8] > { peek (tag ("abcd")) . parse (i) } assert_eq ! (peek_tag (& b"abcdef" [..]) , Ok ((& b"abcdef" [..] , & b"abcd" [..]))) ; assert_eq ! (peek_tag (& b"ab" [..]) , Err (Err :: Incomplete (Needed :: new (2)))) ; assert_eq ! (peek_tag (& b"xxx" [..]) , Err (Err :: Error (error_position ! (& b"xxx" [..] , ErrorKind :: Tag)))) ; }
};
}
