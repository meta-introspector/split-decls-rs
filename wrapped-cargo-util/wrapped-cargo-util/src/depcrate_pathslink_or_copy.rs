// Generated macro for link_or_copy (function)
macro_rules! Depcrate_pathslink_or_copy {
() => {
// Module: crate::paths
// Provides: {"link_or_copy"}
// Dependencies: {}
# [doc = " Hardlink (file) or symlink (dir) src to dst if possible, otherwise copy it."] # [doc = ""] # [doc = " If the destination already exists, it is removed before linking."] pub fn link_or_copy (src : impl AsRef < Path > , dst : impl AsRef < Path >) -> Result < () > { let src = src . as_ref () ; let dst = dst . as_ref () ; _link_or_copy (src , dst) }
};
}
