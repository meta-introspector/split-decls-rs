// Generated macro for test_overflowing_scheme (function)
macro_rules! Depcrate_uri_teststest_overflowing_scheme {
() => {
// Module: crate::uri::tests
// Provides: {"test_overflowing_scheme"}
// Dependencies: {}
# [test] fn test_overflowing_scheme () { let mut uri = vec ! [] ; uri . extend (vec ! [b'a' ; 256]) ; uri . extend (b"://localhost/") ; let uri = String :: from_utf8 (uri) . unwrap () ; let res : Result < Uri , InvalidUri > = uri . parse () ; assert_eq ! (res . unwrap_err () . 0 , ErrorKind :: SchemeTooLong) ; }
};
}
