// Generated macro for AuthenticateFn (type)
macro_rules! Depcrate_remote_connectionAuthenticateFn {
() => {
// Module: crate::remote::connection
// Provides: {"AuthenticateFn"}
// Dependencies: {}
# [doc = " A function that performs a given credential action, trying to obtain credentials for an operation that needs it."] pub type AuthenticateFn < 'a > = Box < dyn FnMut (gix_credentials :: helper :: Action) -> gix_credentials :: protocol :: Result + 'a > ;
};
}
