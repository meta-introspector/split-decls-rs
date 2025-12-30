// Generated macro for impl_31 (impl)
macro_rules! Depcrate_aeadimpl_31 {
() => {
// Module: crate::aead
// Provides: {"impl_31"}
// Dependencies: {}
impl MessageDecrypter for Tls13Cipher { fn decrypt < 'a > (& mut self , mut m : InboundOpaqueMessage < 'a > , seq : u64 ,) -> Result < InboundPlainMessage < 'a > , rustls :: Error > { let payload = & mut m . payload ; let nonce = chacha20poly1305 :: Nonce :: from (Nonce :: new (& self . 1 , seq) . to_array () ?) ; let aad = make_tls13_aad (payload . len ()) ; self . 0 . decrypt_in_place (& nonce , & aad , & mut DecryptBufferAdapter (payload)) . map_err (| _ | rustls :: Error :: DecryptError) ? ; m . into_tls13_unpadded_message () } }
};
}
