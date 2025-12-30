// Generated macro for init (module)
macro_rules! Depcrate_reference_iterinit {
() => {
// Module: crate::reference::iter
// Provides: {"init"}
// Dependencies: {}
# [doc = ""] pub mod init { # [doc = " The error returned by [`Platform::all()`](super::Platform::all()) or [`Platform::prefixed()`](super::Platform::prefixed())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Io (# [from] std :: io :: Error) , # [error (transparent)] RelativePath (# [from] gix_path :: relative_path :: Error) , } }
};
}
