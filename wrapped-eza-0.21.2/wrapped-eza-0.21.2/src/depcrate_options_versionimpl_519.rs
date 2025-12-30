// Generated macro for impl_519 (impl)
macro_rules! Depcrate_options_versionimpl_519 {
() => {
// Module: crate::options::version
// Provides: {"impl_519"}
// Dependencies: {}
impl VersionString { # [doc = " Determines how to show the version, if at all, based on the user’s"] # [doc = " command-line arguments. This one works backwards from the other"] # [doc = " ‘deduce’ functions, returning Err if help needs to be shown."] # [doc = ""] # [doc = " Like --help, this doesn’t check for errors."] pub fn deduce (matches : & MatchedFlags < '_ >) -> Option < Self > { if matches . count (& flags :: VERSION) > 0 { Some (Self) } else { None } } }
};
}
