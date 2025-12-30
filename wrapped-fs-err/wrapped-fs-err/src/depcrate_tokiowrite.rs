// Generated macro for write (function)
macro_rules! Depcrate_tokiowrite {
() => {
// Module: crate::tokio
// Provides: {"write"}
// Dependencies: {}
# [doc = " Creates a future that will open a file for writing and write the entire"] # [doc = " contents of `contents` to it."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::write`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn write (path : impl AsRef < Path > , contents : impl AsRef < [u8] >) -> io :: Result < () > { let (path , contents) = (path . as_ref () , contents . as_ref ()) ; tokio :: fs :: write (path , contents) . await . map_err (| err | Error :: build (err , ErrorKind :: Write , path)) }
};
}
