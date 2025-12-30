// Generated macro for impl_168 (impl)
macro_rules! Depcrate_collections_str_lossyimpl_168 {
() => {
// Module: crate::collections::str::lossy
// Provides: {"impl_168"}
// Dependencies: {}
impl < 'a > Utf8Lossy < 'a > { pub fn from_bytes (bytes : & 'a [u8]) -> Utf8Lossy < 'a > { Utf8Lossy { bytes } } pub fn chunks (& self) -> Utf8LossyChunksIter < 'a > { Utf8LossyChunksIter { source : & self . bytes , } } }
};
}
