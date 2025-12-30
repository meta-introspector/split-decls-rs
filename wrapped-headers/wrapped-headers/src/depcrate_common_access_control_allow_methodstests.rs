// Generated macro for tests (module)
macro_rules! Depcrate_common_access_control_allow_methodstests {
() => {
// Module: crate::common::access_control_allow_methods
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: { test_decode , test_encode } ; use super :: * ; # [test] fn iter () { let allowed = test_decode :: < AccessControlAllowMethods > (& ["GET, PUT"]) . unwrap () ; let as_vec = allowed . iter () . collect :: < Vec < _ > > () ; assert_eq ! (as_vec . len () , 2) ; assert_eq ! (as_vec [0] , Method :: GET) ; assert_eq ! (as_vec [1] , Method :: PUT) ; } # [test] fn from_iter () { let allow : AccessControlAllowMethods = vec ! [Method :: GET , Method :: PUT] . into_iter () . collect () ; let headers = test_encode (allow) ; assert_eq ! (headers ["access-control-allow-methods"] , "GET, PUT") ; } }
};
}
