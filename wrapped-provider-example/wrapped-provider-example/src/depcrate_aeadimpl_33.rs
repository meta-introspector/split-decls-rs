// Generated macro for impl_33 (impl)
macro_rules! Depcrate_aeadimpl_33 {
() => {
// Module: crate::aead
// Provides: {"impl_33"}
// Dependencies: {}
impl MessageEncrypter for Tls12Cipher { fn encrypt (& mut self , m : OutboundPlainMessage < '_ > , seq : u64 ,) -> Result < OutboundOpaqueMessage , rustls :: Error > { let total_len = self . encrypted_payload_len (m . payload . len ()) ; let mut payload = PrefixedPayload :: with_capacity (total_len) ; payload . extend_from_chunks (& m . payload) ; let nonce = chacha20poly1305 :: Nonce :: from (Nonce :: new (& self . 1 , seq) . to_array () ?) ; let aad = make_tls12_aad (seq , m . typ , m . version , m . payload . len ()) ; self . 0 . encrypt_in_place (& nonce , & aad , & mut EncryptBufferAdapter (& mut payload)) . map_err (| _ | rustls :: Error :: EncryptError) . map (| _ | OutboundOpaqueMessage { typ : m . typ , version : m . version , payload , }) } fn encrypted_payload_len (& self , payload_len : usize) -> usize { payload_len + CHACHAPOLY1305_OVERHEAD } }
};
}
