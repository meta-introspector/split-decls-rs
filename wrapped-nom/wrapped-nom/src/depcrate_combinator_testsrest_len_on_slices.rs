// Generated macro for rest_len_on_slices (function)
macro_rules! Depcrate_combinator_testsrest_len_on_slices {
() => {
// Module: crate::combinator::tests
// Provides: {"rest_len_on_slices"}
// Dependencies: {}
# [test] fn rest_len_on_slices () { let input : & [u8] = & b"Hello, world!" [..] ; assert_parse ! (rest_len (input) , Ok ((input , input . len ()))) ; }
};
}
