// Generated macro for rename_section (module)
macro_rules! Depcrate_filerename_section {
() => {
// Module: crate::file
// Provides: {"rename_section"}
// Dependencies: {}
# [doc = ""] pub mod rename_section { # [doc = " The error returned by [`File::rename_section(…)`][crate::File::rename_section()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Lookup (# [from] crate :: lookup :: existing :: Error) , # [error (transparent)] Section (# [from] crate :: parse :: section :: header :: Error) , } }
};
}
