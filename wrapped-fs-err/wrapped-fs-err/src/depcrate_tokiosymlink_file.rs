// Generated macro for symlink_file (function)
macro_rules! Depcrate_tokiosymlink_file {
() => {
// Module: crate::tokio
// Provides: {"symlink_file"}
// Dependencies: {}
# [doc = " Creates a new file symbolic link on the filesystem."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::symlink_file`]."] # [cfg (windows)] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn symlink_file (src : impl AsRef < Path > , dst : impl AsRef < Path >) -> io :: Result < () > { let (src , dst) = (src . as_ref () , dst . as_ref ()) ; tokio :: fs :: symlink_file (src , dst) . await . map_err (| err | SourceDestError :: build (err , SourceDestErrorKind :: SymlinkFile , src , dst)) }
};
}
