// Generated macro for Error (enum)
macro_rules! Depcrate_blob_platform_set_resourceError {
() => {
// Module: crate::blob::platform::set_resource
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [Platform::set_resource](Platform::set_resource)."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Can only diff blobs, not {mode:?}")] InvalidMode { mode : gix_object :: tree :: EntryKind } , # [error ("Failed to read {kind:?} worktree data from '{rela_path}'")] Io { rela_path : BString , kind : ResourceKind , source : std :: io :: Error , } , # [error ("Failed to obtain attributes for {kind:?} resource at '{rela_path}'")] Attributes { rela_path : BString , kind : ResourceKind , source : std :: io :: Error , } , # [error (transparent)] ConvertToMergeable (# [from] pipeline :: convert_to_mergeable :: Error) , }
};
}
