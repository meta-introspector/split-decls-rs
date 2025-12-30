// Generated macro for impl_30 (impl)
macro_rules! Depcrate_aeadimpl_30 {
() => {
// Module: crate::aead
// Provides: {"impl_30"}
// Dependencies: {}
impl MessageEncrypter for Tls13Cipher { fn encrypt (& mut self , m : OutboundPlainMessage < '_ > , seq : u64 ,) -> Result < OutboundOpaqueMessage , rustls :: Error > { let total_len = self . encrypted_payload_len (m . payload . len ()) ; let mut payload = PrefixedPayload :: with_capacity (total_len) ; payload . extend_from_chunks (& m . payload) ; payload . extend_from_slice (& m . typ . to_array ()) ; let nonce = chacha20poly1305 :: Nonce :: from (Nonce :: new (& self . 1 , seq) . to_array () ?) ; let aad = make_tls13_aad (total_len) ; self . 0 . encrypt_in_place (& nonce , & aad , & mut EncryptBufferAdapter (& mut payload)) . map_err (| _ | rustls :: Error :: EncryptError) . map (| _ | OutboundOpaqueMessage { typ : ContentType :: ApplicationData , version : ProtocolVersion :: TLSv1_2 , payload , }) } fn encrypted_payload_len (& self , payload_len : usize) -> usize { payload_len + 1 + CHACHAPOLY1305_OVERHEAD } }
};
}
