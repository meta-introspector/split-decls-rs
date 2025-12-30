// Generated macro for tests (module)
macro_rules! Depcrate_common_access_control_expose_headerstests {
() => {
// Module: crate::common::access_control_expose_headers
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: { test_decode , test_encode } ; use super :: * ; # [test] fn iter () { let expose_headers = test_decode :: < AccessControlExposeHeaders > (& ["foo, bar"]) . unwrap () ; let as_vec = expose_headers . iter () . collect :: < Vec < _ > > () ; assert_eq ! (as_vec . len () , 2) ; assert_eq ! (as_vec [0] , "foo") ; assert_eq ! (as_vec [1] , "bar") ; } # [test] fn from_iter () { let expose : AccessControlExposeHeaders = vec ! [:: http :: header :: CACHE_CONTROL , :: http :: header :: IF_RANGE] . into_iter () . collect () ; let headers = test_encode (expose) ; assert_eq ! (headers ["access-control-expose-headers"] , "cache-control, if-range") ; } }
};
}
