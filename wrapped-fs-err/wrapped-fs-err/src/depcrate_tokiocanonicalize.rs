// Generated macro for canonicalize (function)
macro_rules! Depcrate_tokiocanonicalize {
() => {
// Module: crate::tokio
// Provides: {"canonicalize"}
// Dependencies: {}
# [doc = " Returns the canonical, absolute form of a path with all intermediate"] # [doc = " components normalized and symbolic links resolved."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::canonicalize`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn canonicalize (path : impl AsRef < Path >) -> io :: Result < PathBuf > { let path = path . as_ref () ; tokio :: fs :: canonicalize (path) . await . map_err (| err | Error :: build (err , ErrorKind :: Canonicalize , path)) }
};
}
