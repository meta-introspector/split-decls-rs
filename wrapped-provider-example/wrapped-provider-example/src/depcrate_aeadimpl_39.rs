// Generated macro for impl_39 (impl)
macro_rules! Depcrate_aeadimpl_39 {
() => {
// Module: crate::aead
// Provides: {"impl_39"}
// Dependencies: {}
impl Buffer for EncryptBufferAdapter < '_ > { fn extend_from_slice (& mut self , other : & [u8]) -> chacha20poly1305 :: aead :: Result < () > { self . 0 . extend_from_slice (other) ; Ok (()) } fn truncate (& mut self , len : usize) { self . 0 . truncate (len) } }
};
}
