// Generated macro for Error (enum)
macro_rules! Depcrate_commitError {
() => {
// Module: crate::commit
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`commit(…)`](crate::Repository::commit())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] ParseTime (# [from] crate :: config :: time :: Error) , # [error ("Committer identity is not configured")] CommitterMissing , # [error ("Author identity is not configured")] AuthorMissing , # [error (transparent)] ReferenceNameValidation (# [from] gix_ref :: name :: Error) , # [error (transparent)] WriteObject (# [from] crate :: object :: write :: Error) , # [error (transparent)] ReferenceEdit (# [from] crate :: reference :: edit :: Error) , }
};
}
