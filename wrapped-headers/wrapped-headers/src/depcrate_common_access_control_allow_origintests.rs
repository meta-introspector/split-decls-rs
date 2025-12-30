// Generated macro for tests (module)
macro_rules! Depcrate_common_access_control_allow_origintests {
() => {
// Module: crate::common::access_control_allow_origin
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: { test_decode , test_encode } ; use super :: * ; # [test] fn origin () { let s = "http://web-platform.test:8000" ; let allow_origin = test_decode :: < AccessControlAllowOrigin > (& [s]) . unwrap () ; { let origin = allow_origin . origin () . unwrap () ; assert_eq ! (origin . scheme () , "http") ; assert_eq ! (origin . hostname () , "web-platform.test") ; assert_eq ! (origin . port () , Some (8000)) ; } let headers = test_encode (allow_origin) ; assert_eq ! (headers ["access-control-allow-origin"] , s) ; } # [test] fn try_from_origin () { let s = "http://web-platform.test:8000" ; let allow_origin = AccessControlAllowOrigin :: try_from (s) . unwrap () ; { let origin = allow_origin . origin () . unwrap () ; assert_eq ! (origin . scheme () , "http") ; assert_eq ! (origin . hostname () , "web-platform.test") ; assert_eq ! (origin . port () , Some (8000)) ; } let headers = test_encode (allow_origin) ; assert_eq ! (headers ["access-control-allow-origin"] , s) ; } # [test] fn any () { let allow_origin = test_decode :: < AccessControlAllowOrigin > (& ["*"]) . unwrap () ; assert_eq ! (allow_origin , AccessControlAllowOrigin :: ANY) ; let headers = test_encode (allow_origin) ; assert_eq ! (headers ["access-control-allow-origin"] , "*") ; } # [test] fn null () { let allow_origin = test_decode :: < AccessControlAllowOrigin > (& ["null"]) . unwrap () ; assert_eq ! (allow_origin , AccessControlAllowOrigin :: NULL) ; let headers = test_encode (allow_origin) ; assert_eq ! (headers ["access-control-allow-origin"] , "null") ; } }
};
}
