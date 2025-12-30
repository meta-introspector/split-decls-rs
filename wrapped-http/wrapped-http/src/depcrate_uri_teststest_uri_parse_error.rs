// Generated macro for test_uri_parse_error (function)
macro_rules! Depcrate_uri_teststest_uri_parse_error {
() => {
// Module: crate::uri::tests
// Provides: {"test_uri_parse_error"}
// Dependencies: {}
# [test] fn test_uri_parse_error () { fn err (s : & str) { Uri :: from_str (s) . unwrap_err () ; } err ("http://") ; err ("htt:p//host") ; err ("hyper.rs/") ; err ("hyper.rs?key=val") ; err ("?key=val") ; err ("localhost/") ; err ("localhost?key=val") ; err ("\0") ; err ("http://[::1") ; err ("http://::1]") ; err ("localhost:8080:3030") ; err ("@") ; err ("http://username:password@/wut") ; err ("/?foo\rbar") ; err ("/?foo\nbar") ; err ("/?<") ; err ("/?>") ; }
};
}
