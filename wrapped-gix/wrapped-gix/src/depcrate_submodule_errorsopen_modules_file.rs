// Generated macro for open_modules_file (module)
macro_rules! Depcrate_submodule_errorsopen_modules_file {
() => {
// Module: crate::submodule::errors
// Provides: {"open_modules_file"}
// Dependencies: {}
# [doc = ""] pub mod open_modules_file { # [doc = " The error returned by [Repository::open_modules_file()](crate::Repository::open_modules_file())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Configuration (# [from] gix_config :: parse :: Error) , # [error ("Could not read '.gitmodules' file")] Io (# [from] std :: io :: Error) , } }
};
}
