// Generated macro for stat_options (module)
macro_rules! Depcrate_configstat_options {
() => {
// Module: crate::config
// Provides: {"stat_options"}
// Dependencies: {}
# [doc = ""] pub mod stat_options { # [doc = " The error produced when collecting stat information, and returned by [Repository::stat_options()](crate::Repository::stat_options())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] ConfigCheckStat (# [from] super :: key :: GenericErrorWithValue) , # [error (transparent)] ConfigBoolean (# [from] super :: boolean :: Error) , } }
};
}
