// Generated macro for Error (enum)
macro_rules! Depcrate_treeError {
() => {
// Module: crate::tree
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`tree()`](crate::tree())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not find ancestor, our or their tree to get started")] FindTree (# [from] gix_object :: find :: existing_object :: Error) , # [error ("Could not find ancestor, our or their tree iterator to get started")] FindTreeIter (# [from] gix_object :: find :: existing_iter :: Error) , # [error ("Failed to diff our side or their side")] DiffTree (# [from] gix_diff :: tree_with_rewrites :: Error) , # [error ("Could not apply merge result to base tree")] TreeEdit (# [from] gix_object :: tree :: editor :: Error) , # [error ("Failed to load resource to prepare for blob merge")] BlobMergeSetResource (# [from] crate :: blob :: platform :: set_resource :: Error) , # [error (transparent)] BlobMergePrepare (# [from] crate :: blob :: platform :: prepare_merge :: Error) , # [error (transparent)] BlobMerge (# [from] crate :: blob :: platform :: merge :: Error) , # [error ("Failed to write merged blob content as blob to the object database")] WriteBlobToOdb (Box < dyn std :: error :: Error + Send + Sync + 'static >) , # [error ("The merge was performed, but the binary merge result couldn't be selected as it wasn't found")] MergeResourceNotFound , }
};
}
