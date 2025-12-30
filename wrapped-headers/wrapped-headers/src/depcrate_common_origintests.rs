// Generated macro for tests (module)
macro_rules! Depcrate_common_origintests {
() => {
// Module: crate::common::origin
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: { test_decode , test_encode } ; use super :: * ; # [test] fn origin () { let s = "http://web-platform.test:8000" ; let origin = test_decode :: < Origin > (& [s]) . unwrap () ; assert_eq ! (origin . scheme () , "http") ; assert_eq ! (origin . hostname () , "web-platform.test") ; assert_eq ! (origin . port () , Some (8000)) ; let headers = test_encode (origin) ; assert_eq ! (headers ["origin"] , s) ; } # [test] fn null () { assert_eq ! (test_decode ::< Origin > (& ["null"]) , Some (Origin :: NULL) ,) ; let headers = test_encode (Origin :: NULL) ; assert_eq ! (headers ["origin"] , "null") ; } }
};
}
