// Generated macro for test_max_length_scheme (function)
macro_rules! Depcrate_uri_teststest_max_length_scheme {
() => {
// Module: crate::uri::tests
// Provides: {"test_max_length_scheme"}
// Dependencies: {}
# [test] fn test_max_length_scheme () { let mut uri = vec ! [] ; uri . extend (vec ! [b'a' ; 64]) ; uri . extend (b"://localhost/") ; let uri = String :: from_utf8 (uri) . unwrap () ; let uri : Uri = uri . parse () . unwrap () ; assert_eq ! (uri . scheme_str () . unwrap () . len () , 64) ; }
};
}
