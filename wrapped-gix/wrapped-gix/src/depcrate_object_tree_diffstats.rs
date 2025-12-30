// Generated macro for stats (module)
macro_rules! Depcrate_object_tree_diffstats {
() => {
// Module: crate::object::tree::diff
// Provides: {"stats"}
// Dependencies: {}
# [doc = ""] pub mod stats { # [doc = " The error returned by [`stats()`](super::Platform::stats())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] CreateResourceCache (# [from] crate :: repository :: diff_resource_cache :: Error) , # [error (transparent)] ForEachChange (# [from] crate :: object :: tree :: diff :: for_each :: Error) , } }
};
}
