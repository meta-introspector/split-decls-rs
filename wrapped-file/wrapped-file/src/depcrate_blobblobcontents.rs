// Generated macro for BlobContents (trait)
macro_rules! Depcrate_blobBlobContents {
() => {
// Module: crate::blob
// Provides: {"BlobContents"}
// Dependencies: {}
# [doc = " This trait is used to overload the `Blob::new_with_options` function, allowing a variety of"] # [doc = " types to be used to create a `Blob`. Ignore this, and use &\\[u8], &str, etc to create a `Blob`."] # [doc = ""] # [doc = " The trait is sealed: it can only be implemented by types in this"] # [doc = " crate, as this crate relies on invariants regarding the `JsValue` returned from `into_jsvalue`."] pub trait BlobContents : Sealed { # [doc = " # Safety"] # [doc = ""] # [doc = " For `&[u8]` and `&str`, the returned `Uint8Array` must be modified,"] # [doc = " and must not be kept past the lifetime of the original slice."] unsafe fn into_jsvalue (self) -> JsValue ; }
};
}
