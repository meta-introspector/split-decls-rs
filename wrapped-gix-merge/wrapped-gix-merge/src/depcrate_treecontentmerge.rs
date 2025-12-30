// Generated macro for ContentMerge (struct)
macro_rules! Depcrate_treeContentMerge {
() => {
// Module: crate::tree
// Provides: {"ContentMerge"}
// Dependencies: {}
# [doc = " Information about a blob content merge for use in a [`Resolution`]."] # [doc = " Note that content merges always count as success to avoid duplication of cases, which forces callers"] # [doc = " to check for the [`resolution`](Self::resolution) field."] # [derive (Debug , Copy , Clone)] pub struct ContentMerge { # [doc = " The fully merged blob."] pub merged_blob_id : gix_hash :: ObjectId , # [doc = " Identify the kind of resolution of the blob merge. Note that it may be conflicting."] pub resolution : crate :: blob :: Resolution , }
};
}
