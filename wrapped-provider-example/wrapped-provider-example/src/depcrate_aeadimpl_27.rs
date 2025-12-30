// Generated macro for impl_27 (impl)
macro_rules! Depcrate_aeadimpl_27 {
() => {
// Module: crate::aead
// Provides: {"impl_27"}
// Dependencies: {}
impl Tls13AeadAlgorithm for Chacha20Poly1305 { fn encrypter (& self , key : AeadKey , iv : Iv) -> Box < dyn MessageEncrypter > { Box :: new (Tls13Cipher (chacha20poly1305 :: ChaCha20Poly1305 :: new_from_slice (key . as_ref ()) . unwrap () , iv ,)) } fn decrypter (& self , key : AeadKey , iv : Iv) -> Box < dyn MessageDecrypter > { Box :: new (Tls13Cipher (chacha20poly1305 :: ChaCha20Poly1305 :: new_from_slice (key . as_ref ()) . unwrap () , iv ,)) } fn key_len (& self) -> usize { chacha20poly1305 :: ChaCha20Poly1305 :: key_size () } fn extract_keys (& self , key : AeadKey , iv : Iv ,) -> Result < ConnectionTrafficSecrets , UnsupportedOperationError > { Ok (ConnectionTrafficSecrets :: Chacha20Poly1305 { key , iv }) } }
};
}
