// Generated macro for ignores_whitespace (function)
macro_rules! Depcrate_testsignores_whitespace {
() => {
// Module: crate::tests
// Provides: {"ignores_whitespace"}
// Dependencies: {}
# [test] fn ignores_whitespace () { assert_matches ("1+2" , "fn f() -> i32 {1  +  2}" , & ["1  +  2"]) ; assert_matches ("1 + 2" , "fn f() -> i32 {1+2}" , & ["1+2"]) ; }
};
}
