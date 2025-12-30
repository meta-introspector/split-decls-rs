// Generated macro for Authorization (enum)
macro_rules! Depcrate_hooksAuthorization {
() => {
// Module: crate::hooks
// Provides: {"Authorization"}
// Dependencies: {}
# [doc = " [`authorizer`](Connection::authorizer) return code"] # [derive (Clone , Copy , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum Authorization { # [doc = " Authorize the action."] Allow , # [doc = " Don't allow access, but don't trigger an error either."] Ignore , # [doc = " Trigger an error."] Deny , }
};
}
