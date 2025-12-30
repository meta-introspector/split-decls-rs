// Generated macro for impl_212 (impl)
macro_rules! Depcrate_blobimpl_212 {
() => {
// Module: crate::blob
// Provides: {"impl_212"}
// Dependencies: {}
impl Blob { # [doc = " Provide a `BlobRef` to this owned blob"] pub fn to_ref (& self) -> BlobRef < '_ > { BlobRef { data : & self . data } } }
};
}
