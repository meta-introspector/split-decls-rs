// Generated macro for read_to_string (function)
macro_rules! Depcrate_tokioread_to_string {
() => {
// Module: crate::tokio
// Provides: {"read_to_string"}
// Dependencies: {}
# [doc = " Creates a future which will open a file for reading and read the entire"] # [doc = " contents into a string and return said string."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::read_to_string`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn read_to_string (path : impl AsRef < Path >) -> io :: Result < String > { let path = path . as_ref () ; tokio :: fs :: read_to_string (path) . await . map_err (| err | Error :: build (err , ErrorKind :: Read , path)) }
};
}
