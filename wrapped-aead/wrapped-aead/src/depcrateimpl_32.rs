// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T : AeadInOut > Aead for T { fn encrypt < 'msg , 'aad > (& self , nonce : & Nonce < Self > , plaintext : impl Into < Payload < 'msg , 'aad > > ,) -> Result < Vec < u8 > > { let payload = plaintext . into () ; let mut buffer = Vec :: with_capacity (payload . msg . len () + Self :: TagSize :: to_usize ()) ; buffer . extend_from_slice (payload . msg) ; self . encrypt_in_place (nonce , payload . aad , & mut buffer) ? ; Ok (buffer) } fn decrypt < 'msg , 'aad > (& self , nonce : & Nonce < Self > , ciphertext : impl Into < Payload < 'msg , 'aad > > ,) -> Result < Vec < u8 > > { let payload = ciphertext . into () ; let mut buffer = Vec :: from (payload . msg) ; self . decrypt_in_place (nonce , payload . aad , & mut buffer) ? ; Ok (buffer) } }
};
}
