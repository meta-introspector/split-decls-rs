// Generated macro for read_dir (function)
macro_rules! Depcrate_tokio_read_dirread_dir {
() => {
// Module: crate::tokio::read_dir
// Provides: {"read_dir"}
// Dependencies: {}
# [doc = " Returns a stream over the entries within a directory."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::read_dir`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn read_dir (path : impl AsRef < Path >) -> io :: Result < ReadDir > { let path = path . as_ref () ; let tokio = fs :: read_dir (path) . await . map_err (| err | Error :: build (err , ErrorKind :: ReadDir , path)) ? ; Ok (ReadDir { tokio , path : path . to_owned () , }) }
};
}
