// Generated macro for Error (enum)
macro_rules! Depcrate_blob_platform_prepare_mergeError {
() => {
// Module: crate::blob::platform::prepare_merge
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [Platform::prepare_merge_state()](Platform::prepare_merge())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The 'current', 'ancestor' or 'other' resource for the merge operation were not set")] UnsetResource , # [error ("Failed to obtain attributes for {kind:?} resource at '{rela_path}'")] Attributes { rela_path : BString , kind : ResourceKind , source : std :: io :: Error , } , }
};
}
