// Generated macro for symlink_dir (function)
macro_rules! Depcrate_tokiosymlink_dir {
() => {
// Module: crate::tokio
// Provides: {"symlink_dir"}
// Dependencies: {}
# [doc = " Creates a new directory symlink on the filesystem."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::symlink_dir`]."] # [cfg (windows)] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn symlink_dir (src : impl AsRef < Path > , dst : impl AsRef < Path >) -> io :: Result < () > { let (src , dst) = (src . as_ref () , dst . as_ref ()) ; tokio :: fs :: symlink_dir (src , dst) . await . map_err (| err | SourceDestError :: build (err , SourceDestErrorKind :: SymlinkDir , src , dst)) }
};
}
