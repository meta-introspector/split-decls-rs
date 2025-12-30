// Generated macro for Error (enum)
macro_rules! Depcrate_file_commitError {
() => {
// Module: crate::file::commit
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error used in the [`file::commit`][self] module."] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("commit {0}'s extra edges overflows the commit-graph file's extra edges list")] ExtraEdgesListOverflow (gix_hash :: ObjectId) , # [error ("commit {0}'s first parent is an extra edge index, which is invalid")] FirstParentIsExtraEdgeIndex (gix_hash :: ObjectId) , # [error ("commit {0} has extra edges, but commit-graph file has no extra edges list")] MissingExtraEdgesList (gix_hash :: ObjectId) , # [error ("commit {0} has a second parent but not a first parent")] SecondParentWithoutFirstParent (gix_hash :: ObjectId) , }
};
}
