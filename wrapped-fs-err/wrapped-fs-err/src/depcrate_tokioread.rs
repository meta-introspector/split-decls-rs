// Generated macro for read (function)
macro_rules! Depcrate_tokioread {
() => {
// Module: crate::tokio
// Provides: {"read"}
// Dependencies: {}
# [doc = " Reads the entire contents of a file into a bytes vector."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::read`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn read (path : impl AsRef < Path >) -> io :: Result < Vec < u8 > > { let path = path . as_ref () ; tokio :: fs :: read (path) . await . map_err (| err | Error :: build (err , ErrorKind :: Read , path)) }
};
}
