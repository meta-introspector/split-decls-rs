// Generated macro for impl_7 (impl)
macro_rules! Depcrate_blobimpl_7 {
() => {
// Module: crate::blob
// Provides: {"impl_7"}
// Dependencies: {}
impl < 'a > BlobContents for & 'a str { unsafe fn into_jsvalue (self) -> JsValue { self . as_bytes () . into_jsvalue () } }
};
}
