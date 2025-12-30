// Generated macro for test_authority_uri_parts_round_trip (function)
macro_rules! Depcrate_uri_teststest_authority_uri_parts_round_trip {
() => {
// Module: crate::uri::tests
// Provides: {"test_authority_uri_parts_round_trip"}
// Dependencies: {}
# [test] fn test_authority_uri_parts_round_trip () { let s = "hyper.rs" ; let uri = Uri :: from_str (s) . expect ("first parse") ; assert_eq ! (uri , s) ; assert_eq ! (uri . to_string () , s) ; let parts = uri . into_parts () ; let uri2 = Uri :: from_parts (parts) . expect ("from_parts") ; assert_eq ! (uri2 , s) ; assert_eq ! (uri2 . to_string () , s) ; }
};
}
