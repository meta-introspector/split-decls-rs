// Generated macro for convert_to_mergeable (module)
macro_rules! Depcrate_blob_pipelineconvert_to_mergeable {
() => {
// Module: crate::blob::pipeline
// Provides: {"convert_to_mergeable"}
// Dependencies: {}
# [doc = ""] pub mod convert_to_mergeable { use std :: collections :: TryReserveError ; use bstr :: BString ; use gix_object :: tree :: EntryKind ; # [doc = " The error returned by [Pipeline::convert_to_mergeable()](super::Pipeline::convert_to_mergeable())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Entry at '{rela_path}' must be regular file or symlink, but was {actual:?}")] InvalidEntryKind { rela_path : BString , actual : EntryKind } , # [error ("Entry at '{rela_path}' could not be read as symbolic link")] ReadLink { rela_path : BString , source : std :: io :: Error } , # [error ("Entry at '{rela_path}' could not be opened for reading or read from")] OpenOrRead { rela_path : BString , source : std :: io :: Error } , # [error ("Entry at '{rela_path}' could not be copied from a filter process to a memory buffer")] StreamCopy { rela_path : BString , source : std :: io :: Error } , # [error (transparent)] FindObject (# [from] gix_object :: find :: existing_object :: Error) , # [error (transparent)] ConvertToWorktree (# [from] gix_filter :: pipeline :: convert :: to_worktree :: Error) , # [error (transparent)] ConvertToGit (# [from] gix_filter :: pipeline :: convert :: to_git :: Error) , # [error ("Memory allocation failed")] OutOfMemory (# [from] TryReserveError) , } }
};
}
