// Generated macro for test_uri_to_path_and_query (function)
macro_rules! Depcrate_uri_teststest_uri_to_path_and_query {
() => {
// Module: crate::uri::tests
// Provides: {"test_uri_to_path_and_query"}
// Dependencies: {}
# [test] fn test_uri_to_path_and_query () { let cases = vec ! [("/" , "/") , ("/foo?bar" , "/foo?bar") , ("/foo?bar#nope" , "/foo?bar") , ("http://hyper.rs" , "/") , ("http://hyper.rs/" , "/") , ("http://hyper.rs/path" , "/path") , ("http://hyper.rs?query" , "/?query") , ("*" , "*") ,] ; for case in cases { let uri = Uri :: from_str (case . 0) . unwrap () ; let s = uri . path_and_query () . unwrap () . to_string () ; assert_eq ! (s , case . 1) ; } }
};
}
