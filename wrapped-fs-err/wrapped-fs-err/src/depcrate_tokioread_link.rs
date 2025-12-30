// Generated macro for read_link (function)
macro_rules! Depcrate_tokioread_link {
() => {
// Module: crate::tokio
// Provides: {"read_link"}
// Dependencies: {}
# [doc = " Reads a symbolic link, returning the file that the link points to."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::read_link`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn read_link (path : impl AsRef < Path >) -> io :: Result < PathBuf > { let path = path . as_ref () ; tokio :: fs :: read_link (path) . await . map_err (| err | Error :: build (err , ErrorKind :: ReadLink , path)) }
};
}
