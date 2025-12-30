// Generated macro for impl_43 (impl)
macro_rules! Depcrate_aeadimpl_43 {
() => {
// Module: crate::aead
// Provides: {"impl_43"}
// Dependencies: {}
impl Buffer for DecryptBufferAdapter < '_ , '_ > { fn extend_from_slice (& mut self , _ : & [u8]) -> chacha20poly1305 :: aead :: Result < () > { unreachable ! ("not used by `AeadInPlace::decrypt_in_place`") } fn truncate (& mut self , len : usize) { self . 0 . truncate (len) } }
};
}
