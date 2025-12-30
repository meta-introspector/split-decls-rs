// Generated macro for pathspec_defaults_ignore_case (module)
macro_rules! Depcrate_repositorypathspec_defaults_ignore_case {
() => {
// Module: crate::repository
// Provides: {"pathspec_defaults_ignore_case"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "attributes")] pub mod pathspec_defaults_ignore_case { # [doc = " The error returned by [Repository::pathspec_defaults_ignore_case()](crate::Repository::pathspec_defaults_inherit_ignore_case())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Filesystem configuration could not be obtained to learn about case sensitivity")] FilesystemConfig (# [from] crate :: config :: boolean :: Error) , # [error (transparent)] Defaults (# [from] gix_pathspec :: defaults :: from_environment :: Error) , } }
};
}
