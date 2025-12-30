// Generated macro for symlink_metadata (function)
macro_rules! Depcrate_pathssymlink_metadata {
() => {
// Module: crate::paths
// Provides: {"symlink_metadata"}
// Dependencies: {}
# [doc = " Returns metadata for a file without following symlinks."] # [doc = ""] # [doc = " Equivalent to [`std::fs::metadata`] with better error messages."] pub fn symlink_metadata < P : AsRef < Path > > (path : P) -> Result < Metadata > { let path = path . as_ref () ; std :: fs :: symlink_metadata (path) . with_context (| | format ! ("failed to load metadata for path `{}`" , path . display ())) }
};
}
