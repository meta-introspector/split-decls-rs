// Generated macro for impl_721 (impl)
macro_rules! Depcrate_open_optionsimpl_721 {
() => {
// Module: crate::open::options
// Provides: {"impl_721"}
// Dependencies: {}
# [doc = " Instantiation"] impl Options { # [doc = " Options configured to prevent accessing anything else than the repository configuration file, prohibiting"] # [doc = " accessing the environment or spreading beyond the git repository location."] pub fn isolated () -> Self { Options :: default () . permissions (Permissions :: isolated ()) } }
};
}
