// Generated macro for impl_581 (impl)
macro_rules! Depcrate_cipherimpl_581 {
() => {
// Module: crate::cipher
// Provides: {"impl_581"}
// Dependencies: {}
impl From < hkdf :: Okm < '_ , & 'static Algorithm > > for UnboundCipherKey { fn from (okm : hkdf :: Okm < & 'static Algorithm >) -> Self { let mut key_bytes = [0 ; MAX_CIPHER_KEY_LEN] ; let key_bytes = & mut key_bytes [.. okm . len () . key_len] ; let algorithm = * okm . len () ; okm . fill (key_bytes) . unwrap () ; Self :: new (algorithm , key_bytes) . unwrap () } }
};
}
