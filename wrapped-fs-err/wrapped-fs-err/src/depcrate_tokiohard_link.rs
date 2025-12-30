// Generated macro for hard_link (function)
macro_rules! Depcrate_tokiohard_link {
() => {
// Module: crate::tokio
// Provides: {"hard_link"}
// Dependencies: {}
# [doc = " Creates a new hard link on the filesystem."] # [doc = ""] # [doc = " Wrapper for [`tokio::fs::hard_link`]."] # [cfg_attr (docsrs , doc (cfg (feature = "tokio")))] pub async fn hard_link (src : impl AsRef < Path > , dst : impl AsRef < Path >) -> io :: Result < () > { let (src , dst) = (src . as_ref () , dst . as_ref ()) ; tokio :: fs :: hard_link (src , dst) . await . map_err (| err | SourceDestError :: build (err , SourceDestErrorKind :: HardLink , src , dst)) }
};
}
