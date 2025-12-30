// Generated macro for eof_on_slices (function)
macro_rules! Depcrate_combinator_testseof_on_slices {
() => {
// Module: crate::combinator::tests
// Provides: {"eof_on_slices"}
// Dependencies: {}
# [test] fn eof_on_slices () { let not_over : & [u8] = & b"Hello, world!" [..] ; let is_over : & [u8] = & b"" [..] ; let res_not_over = eof (not_over) ; assert_parse ! (res_not_over , Err (Err :: Error (error_position ! (not_over , ErrorKind :: Eof)))) ; let res_over = eof (is_over) ; assert_parse ! (res_over , Ok ((is_over , is_over))) ; }
};
}
