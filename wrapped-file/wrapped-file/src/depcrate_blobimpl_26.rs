// Generated macro for impl_26 (impl)
macro_rules! Depcrate_blobimpl_26 {
() => {
// Module: crate::blob
// Provides: {"impl_26"}
// Dependencies: {}
impl From < web_sys :: File > for File { fn from (file : web_sys :: File) -> Self { File { inner : Blob :: from (web_sys :: Blob :: from (file)) , } } }
};
}
