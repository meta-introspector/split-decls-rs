// Generated macro for impl_658 (impl)
macro_rules! Depcrate_diffimpl_658 {
() => {
// Module: crate::diff
// Provides: {"impl_658"}
// Dependencies: {}
# [doc = " Builder"] impl Options { # [doc = " Provide `None` to disable rewrite tracking entirely, or pass `Some(<configuration>)` to control to"] # [doc = " what extent rename and copy tracking is performed."] # [doc = ""] # [doc = " Note that by default, the git configuration determines rewrite tracking and git defaults are used"] # [doc = " if nothing is configured, which turns rename tracking with 50% similarity on, while not tracking copies at all."] # [cfg (feature = "blob-diff")] pub fn with_rewrites (mut self , renames : Option < gix_diff :: Rewrites >) -> Self { self . rewrites = renames ; self } }
};
}
