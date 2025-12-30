// Generated macro for impl_9 (impl)
macro_rules! Depcrate_blobimpl_9 {
() => {
// Module: crate::blob
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a > BlobContents for & 'a [u8] { unsafe fn into_jsvalue (self) -> JsValue { js_sys :: Uint8Array :: view (self) . into () } }
};
}
