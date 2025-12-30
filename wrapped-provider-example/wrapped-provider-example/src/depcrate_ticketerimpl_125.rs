// Generated macro for impl_125 (impl)
macro_rules! Depcrate_ticketerimpl_125 {
() => {
// Module: crate::ticketer
// Provides: {"impl_125"}
// Dependencies: {}
impl TicketProducer for AeadTicketer { # [doc = " Encrypt `message` and return the ciphertext."] fn encrypt (& self , message : & [u8]) -> Option < Vec < u8 > > { let mut nonce = Nonce :: default () ; OsRng . fill_bytes (nonce . as_mut_slice ()) ; let mut ciphertext = Vec :: with_capacity (self . key_name . len () + nonce . len () + message . len () + < ChaCha20Poly1305 as AeadCore > :: TagSize :: to_usize () ,) ; ciphertext . extend (self . key_name) ; ciphertext . extend (nonce) ; ciphertext . extend (message) ; let tag = self . key . encrypt_in_place_detached (& nonce , & self . key_name , & mut ciphertext [self . key_name . len () + nonce . len () ..] ,) . ok () ? ; ciphertext . extend (tag . as_slice ()) ; self . maximum_ciphertext_len . fetch_max (ciphertext . len () , Ordering :: SeqCst) ; Some (ciphertext) } fn decrypt (& self , ciphertext : & [u8]) -> Option < Vec < u8 > > { if ciphertext . len () > self . maximum_ciphertext_len . load (Ordering :: SeqCst) { return None ; } let (alleged_key_name , ciphertext) = ciphertext . split_at_checked (self . key_name . len ()) ? ; let (nonce , ciphertext) = ciphertext . split_at_checked (< ChaCha20Poly1305 as AeadCore > :: NonceSize :: to_usize ()) ? ; if ConstantTimeEq :: ct_ne (& self . key_name [..] , alleged_key_name) . into () { return None ; } let nonce = Nonce :: from_slice (nonce) ; let mut out = Vec :: from (ciphertext) ; self . key . decrypt_in_place (nonce , alleged_key_name , & mut out) . ok () ? ; Some (out) } fn lifetime (& self) -> Duration { Duration :: ZERO } }
};
}
