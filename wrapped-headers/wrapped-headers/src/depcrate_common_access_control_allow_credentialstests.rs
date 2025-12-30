// Generated macro for tests (module)
macro_rules! Depcrate_common_access_control_allow_credentialstests {
() => {
// Module: crate::common::access_control_allow_credentials
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: test_decode ; use super :: * ; # [test] fn allow_credentials_is_case_sensitive () { let allow_header = test_decode :: < AccessControlAllowCredentials > (& ["true"]) ; assert ! (allow_header . is_some ()) ; let allow_header = test_decode :: < AccessControlAllowCredentials > (& ["True"]) ; assert ! (allow_header . is_none ()) ; } }
};
}
