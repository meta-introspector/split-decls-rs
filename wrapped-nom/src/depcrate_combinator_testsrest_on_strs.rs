// Generated macro for rest_on_strs (function)
macro_rules! Depcrate_combinator_testsrest_on_strs {
() => {
// Module: crate::combinator::tests
// Provides: {"rest_on_strs"}
// Dependencies: {}
# [test] fn rest_on_strs () { let input : & str = "Hello, world!" ; let empty : & str = "" ; assert_parse ! (rest (input) , Ok ((empty , input))) ; }
};
}
