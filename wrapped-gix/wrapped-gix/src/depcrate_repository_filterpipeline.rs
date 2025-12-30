// Generated macro for pipeline (module)
macro_rules! Depcrate_repository_filterpipeline {
() => {
// Module: crate::repository::filter
// Provides: {"pipeline"}
// Dependencies: {}
# [doc = ""] pub mod pipeline { # [doc = " The error returned by [Repository::filter_pipeline()](super::Repository::filter_pipeline())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not obtain head commit of bare repository")] HeadCommit (# [from] crate :: reference :: head_commit :: Error) , # [error (transparent)] DecodeCommit (# [from] gix_object :: decode :: Error) , # [error ("Could not create index from tree at HEAD^{{tree}}")] TreeTraverse (# [from] crate :: repository :: index_from_tree :: Error) , # [error (transparent)] BareAttributes (# [from] crate :: config :: attribute_stack :: Error) , # [error (transparent)] WorktreeIndex (# [from] crate :: worktree :: open_index :: Error) , # [error (transparent)] Init (# [from] crate :: filter :: pipeline :: options :: Error) , } }
};
}
