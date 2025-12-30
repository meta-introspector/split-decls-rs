// Generated macro for tests (module)
macro_rules! Depcrate_common_access_control_allow_headerstests {
() => {
// Module: crate::common::access_control_allow_headers
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: { test_decode , test_encode } ; use super :: * ; # [test] fn iter () { let allow_headers = test_decode :: < AccessControlAllowHeaders > (& ["foo, bar"]) . unwrap () ; let as_vec = allow_headers . iter () . collect :: < Vec < _ > > () ; assert_eq ! (as_vec . len () , 2) ; assert_eq ! (as_vec [0] , "foo") ; assert_eq ! (as_vec [1] , "bar") ; } # [test] fn from_iter () { let allow : AccessControlAllowHeaders = vec ! [:: http :: header :: CACHE_CONTROL , :: http :: header :: IF_RANGE] . into_iter () . collect () ; let headers = test_encode (allow) ; assert_eq ! (headers ["access-control-allow-headers"] , "cache-control, if-range") ; } # [test] fn test_with_invalid () { let allow_headers = test_decode :: < AccessControlAllowHeaders > (& ["foo foo, bar"]) . unwrap () ; assert ! (allow_headers . iter () . collect ::< Vec < _ >> () . is_empty ()) ; } }
};
}
