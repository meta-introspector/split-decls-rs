// Generated macro for impl_61 (impl)
macro_rules! Depcrate_hazmatimpl_61 {
() => {
// Module: crate::hazmat
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'a > Mode < 'a > { fn key_words (& self) -> CVWords { match self { Mode :: Hash => * IV , Mode :: KeyedHash (key) => crate :: platform :: words_from_le_bytes_32 (key) , Mode :: DeriveKeyMaterial (cx_key) => crate :: platform :: words_from_le_bytes_32 (cx_key) , } } fn flags_byte (& self) -> u8 { match self { Mode :: Hash => 0 , Mode :: KeyedHash (_) => crate :: KEYED_HASH , Mode :: DeriveKeyMaterial (_) => crate :: DERIVE_KEY_MATERIAL , } } }
};
}
