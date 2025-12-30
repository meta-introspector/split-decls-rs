// Generated macro for rename (function)
macro_rules! Depcrate_tokiorename {
() => {
// Module: crate::tokio
// Provides: {"rename"}
// Dependencies: {}
# [doc = " Renames a file or directory to a new name, replacing the original file if"] # [doc = " `to` already exists."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::rename`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn rename (from : impl AsRef < Path > , to : impl AsRef < Path >) -> io :: Result < () > { let (from , to) = (from . as_ref () , to . as_ref ()) ; tokio :: fs :: rename (from , to) . await . map_err (| err | SourceDestError :: build (err , SourceDestErrorKind :: Rename , from , to)) }
};
}
