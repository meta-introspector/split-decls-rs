// Generated macro for path (module)
macro_rules! Depcrate_configpath {
() => {
// Module: crate::config
// Provides: {"path"}
// Dependencies: {}
# [doc = ""] pub mod path { use bstr :: BString ; # [doc = " The error returned by [File::path()](crate::File::path)."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The path '{actual}' of submodule '{submodule}' needs to be relative")] Absolute { actual : BString , submodule : BString } , # [error ("The submodule '{submodule}' was missing its 'path' field or it was empty")] Missing { submodule : BString } , # [error ("The path '{actual}' would lead outside of the repository worktree")] OutsideOfWorktree { actual : BString , submodule : BString } , } }
};
}
