// Generated macro for Error (enum)
macro_rules! Depcrate_config_overridesError {
() => {
// Module: crate::config::overrides
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`SnapshotMut::apply_cli_overrides()`][crate::config::SnapshotMut::append_config()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("{input:?} is not a valid configuration key. Examples are 'core.abbrev' or 'remote.origin.url'")] InvalidKey { input : BString } , # [error ("Key {key:?} could not be parsed")] SectionKey { key : BString , source : gix_config :: parse :: section :: value_name :: Error , } , # [error (transparent)] SectionHeader (# [from] gix_config :: parse :: section :: header :: Error) , }
};
}
