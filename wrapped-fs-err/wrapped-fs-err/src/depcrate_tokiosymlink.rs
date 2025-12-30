// Generated macro for symlink (function)
macro_rules! Depcrate_tokiosymlink {
() => {
// Module: crate::tokio
// Provides: {"symlink"}
// Dependencies: {}
# [doc = " Creates a new symbolic link on the filesystem."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::symlink`]."] # [cfg (unix)] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn symlink (src : impl AsRef < Path > , dst : impl AsRef < Path >) -> io :: Result < () > { let (src , dst) = (src . as_ref () , dst . as_ref ()) ; tokio :: fs :: symlink (src , dst) . await . map_err (| err | SourceDestError :: build (err , SourceDestErrorKind :: Symlink , src , dst)) }
};
}
