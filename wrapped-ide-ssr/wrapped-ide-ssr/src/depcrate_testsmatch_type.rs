// Generated macro for match_type (function)
macro_rules! Depcrate_testsmatch_type {
() => {
// Module: crate::tests
// Provides: {"match_type"}
// Dependencies: {}
# [test] fn match_type () { assert_matches ("i32" , "fn f() -> i32 {1  +  2}" , & ["i32"]) ; assert_matches ("Option<$a>" , "struct Option<T> {} fn f() -> Option<i32> {42}" , & ["Option<i32>"] ,) ; assert_no_match ("Option<$a>" , "struct Option<T> {} struct Result<T, E> {} fn f() -> Result<i32, ()> {42}" ,) ; }
};
}
