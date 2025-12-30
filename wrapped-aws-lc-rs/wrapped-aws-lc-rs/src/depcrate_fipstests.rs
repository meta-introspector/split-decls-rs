// Generated macro for tests (module)
macro_rules! Depcrate_fipstests {
() => {
// Module: crate::fips
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [cfg (all (feature = "fips" , debug_assertions))] # [test] fn test_service_status () { use crate :: fips :: FipsServiceStatus ; assert_eq ! (FipsServiceStatus :: Approved (true) , FipsServiceStatus :: Approved (()) . map (| () | true)) ; assert_eq ! (FipsServiceStatus :: NonApproved (true) , FipsServiceStatus :: NonApproved (()) . map (| () | true)) ; assert_eq ! (FipsServiceStatus :: Unset (true) , FipsServiceStatus :: Unset (()) . map (| () | true)) ; } }
};
}
