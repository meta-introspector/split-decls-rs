// Generated macro for copy (function)
macro_rules! Depcrate_tokiocopy {
() => {
// Module: crate::tokio
// Provides: {"copy"}
// Dependencies: {}
# [doc = " Copies the contents of one file to another. This function will also copy the permission bits"] # [doc = " of the original file to the destination file."] # [doc = " This function will overwrite the contents of to."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::copy`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn copy (from : impl AsRef < Path > , to : impl AsRef < Path >) -> Result < u64 , io :: Error > { let (from , to) = (from . as_ref () , to . as_ref ()) ; tokio :: fs :: copy (from , to) . await . map_err (| err | SourceDestError :: build (err , SourceDestErrorKind :: Copy , from , to)) }
};
}
