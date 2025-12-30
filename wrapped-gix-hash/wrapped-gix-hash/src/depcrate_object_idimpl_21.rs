// Generated macro for impl_21 (impl)
macro_rules! Depcrate_object_idimpl_21 {
() => {
// Module: crate::object_id
// Provides: {"impl_21"}
// Dependencies: {}
# [doc = " Lifecycle"] impl ObjectId { # [doc = " Convert `bytes` into an owned object Id or panic if the slice length doesn't indicate a supported hash."] # [doc = ""] # [doc = " Use `Self::try_from(bytes)` for a fallible version."] pub fn from_bytes_or_panic (bytes : & [u8]) -> Self { match bytes . len () { 20 => Self :: Sha1 (bytes . try_into () . expect ("prior length validation")) , other => panic ! ("BUG: unsupported hash len: {other}") , } } }
};
}
