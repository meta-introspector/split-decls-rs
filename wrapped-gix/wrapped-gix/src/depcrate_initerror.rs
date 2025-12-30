// Generated macro for Error (enum)
macro_rules! Depcrate_initError {
() => {
// Module: crate::init
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`crate::init()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not obtain the current directory")] CurrentDir (# [from] std :: io :: Error) , # [error (transparent)] Init (# [from] crate :: create :: Error) , # [error (transparent)] Open (# [from] crate :: open :: Error) , # [error ("Invalid default branch name: {name:?}")] InvalidBranchName { name : BString , source : gix_validate :: reference :: name :: Error , } , # [error ("Could not edit HEAD reference with new default name")] EditHeadForDefaultBranch (# [from] crate :: reference :: edit :: Error) , }
};
}
