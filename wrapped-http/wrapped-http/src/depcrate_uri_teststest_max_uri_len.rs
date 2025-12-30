// Generated macro for test_max_uri_len (function)
macro_rules! Depcrate_uri_teststest_max_uri_len {
() => {
// Module: crate::uri::tests
// Provides: {"test_max_uri_len"}
// Dependencies: {}
# [test] fn test_max_uri_len () { let mut uri = vec ! [] ; uri . extend (b"http://localhost/") ; uri . extend (vec ! [b'a' ; 70 * 1024]) ; let uri = String :: from_utf8 (uri) . unwrap () ; let res : Result < Uri , InvalidUri > = uri . parse () ; assert_eq ! (res . unwrap_err () . 0 , ErrorKind :: TooLong) ; }
};
}
