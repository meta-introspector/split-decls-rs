// Generated macro for impl_28 (impl)
macro_rules! Depcrate_aeadimpl_28 {
() => {
// Module: crate::aead
// Provides: {"impl_28"}
// Dependencies: {}
impl Tls12AeadAlgorithm for Chacha20Poly1305 { fn encrypter (& self , key : AeadKey , iv : & [u8] , _ : & [u8]) -> Box < dyn MessageEncrypter > { Box :: new (Tls12Cipher (chacha20poly1305 :: ChaCha20Poly1305 :: new_from_slice (key . as_ref ()) . unwrap () , Iv :: new (iv) . expect ("IV length validated by key_block_shape") ,)) } fn decrypter (& self , key : AeadKey , iv : & [u8]) -> Box < dyn MessageDecrypter > { Box :: new (Tls12Cipher (chacha20poly1305 :: ChaCha20Poly1305 :: new_from_slice (key . as_ref ()) . unwrap () , Iv :: new (iv) . expect ("IV length validated by key_block_shape") ,)) } fn key_block_shape (& self) -> KeyBlockShape { KeyBlockShape { enc_key_len : 32 , fixed_iv_len : 12 , explicit_nonce_len : 0 , } } fn extract_keys (& self , key : AeadKey , iv : & [u8] , _explicit : & [u8] ,) -> Result < ConnectionTrafficSecrets , UnsupportedOperationError > { debug_assert_eq ! (NONCE_LEN , iv . len ()) ; Ok (ConnectionTrafficSecrets :: Chacha20Poly1305 { key , iv : Iv :: new (iv) . expect ("IV length validated by key_block_shape") , }) } }
};
}
