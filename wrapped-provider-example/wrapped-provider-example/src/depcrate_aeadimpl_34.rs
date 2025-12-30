// Generated macro for impl_34 (impl)
macro_rules! Depcrate_aeadimpl_34 {
() => {
// Module: crate::aead
// Provides: {"impl_34"}
// Dependencies: {}
impl MessageDecrypter for Tls12Cipher { fn decrypt < 'a > (& mut self , mut m : InboundOpaqueMessage < 'a > , seq : u64 ,) -> Result < InboundPlainMessage < 'a > , rustls :: Error > { let payload = & m . payload ; let nonce = chacha20poly1305 :: Nonce :: from (Nonce :: new (& self . 1 , seq) . to_array () ?) ; let aad = make_tls12_aad (seq , m . typ , m . version , payload . len () - CHACHAPOLY1305_OVERHEAD ,) ; let payload = & mut m . payload ; self . 0 . decrypt_in_place (& nonce , & aad , & mut DecryptBufferAdapter (payload)) . map_err (| _ | rustls :: Error :: DecryptError) ? ; Ok (m . into_plain_message ()) } }
};
}
