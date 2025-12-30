// Generated macro for blob_merge_options (module)
macro_rules! Depcrate_repositoryblob_merge_options {
() => {
// Module: crate::repository
// Provides: {"blob_merge_options"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "merge")] pub mod blob_merge_options { # [doc = " The error returned by [Repository::blob_merge_options()](crate::Repository::blob_merge_options())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] DiffAlgorithm (# [from] crate :: config :: diff :: algorithm :: Error) , # [error (transparent)] ConflictStyle (# [from] crate :: config :: key :: GenericErrorWithValue) , } }
};
}
