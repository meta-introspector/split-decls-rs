// Generated macro for impl_737 (impl)
macro_rules! Depcrate_open_permissionsimpl_737 {
() => {
// Module: crate::open::permissions
// Provides: {"impl_737"}
// Dependencies: {}
impl Permissions { # [doc = " Secure permissions are similar to `all()`"] pub fn secure () -> Self { Permissions { env : Environment :: all () , config : Config :: all () , attributes : Attributes :: all () , } } # [doc = " Everything is allowed with this set of permissions, thus we read all configuration and do what git typically"] # [doc = " does with owned repositories."] pub fn all () -> Self { Permissions { env : Environment :: all () , config : Config :: all () , attributes : Attributes :: all () , } } # [doc = " Don't read any but the local git configuration and deny reading any environment variables."] pub fn isolated () -> Self { Permissions { config : Config :: isolated () , attributes : Attributes :: isolated () , env : Environment :: isolated () , } } }
};
}
