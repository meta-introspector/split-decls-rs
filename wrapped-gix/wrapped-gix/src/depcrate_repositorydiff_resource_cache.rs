// Generated macro for diff_resource_cache (module)
macro_rules! Depcrate_repositorydiff_resource_cache {
() => {
// Module: crate::repository
// Provides: {"diff_resource_cache"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "blob-diff")] pub mod diff_resource_cache { # [doc = " The error returned by [Repository::diff_resource_cache()](crate::Repository::diff_resource_cache())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not obtain resource cache for diffing")] ResourceCache (# [from] crate :: diff :: resource_cache :: Error) , # [error (transparent)] Index (# [from] crate :: repository :: index_or_load_from_head_or_empty :: Error) , # [error (transparent)] AttributeStack (# [from] crate :: config :: attribute_stack :: Error) , } }
};
}
