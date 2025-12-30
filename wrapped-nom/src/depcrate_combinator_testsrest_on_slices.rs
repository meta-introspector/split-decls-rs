// Generated macro for rest_on_slices (function)
macro_rules! Depcrate_combinator_testsrest_on_slices {
() => {
// Module: crate::combinator::tests
// Provides: {"rest_on_slices"}
// Dependencies: {}
# [test] fn rest_on_slices () { let input : & [u8] = & b"Hello, world!" [..] ; let empty : & [u8] = & b"" [..] ; assert_parse ! (rest (input) , Ok ((empty , input))) ; }
};
}
