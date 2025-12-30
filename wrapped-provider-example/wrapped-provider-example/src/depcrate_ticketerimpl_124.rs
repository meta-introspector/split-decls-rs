// Generated macro for impl_124 (impl)
macro_rules! Depcrate_ticketerimpl_124 {
() => {
// Module: crate::ticketer
// Provides: {"impl_124"}
// Dependencies: {}
impl AeadTicketer { # [expect (clippy :: new_ret_no_self)] pub (super) fn new () -> Result < Box < dyn TicketProducer > , Error > { let mut key = Key :: default () ; OsRng . fill_bytes (key . as_mut_slice ()) ; let mut key_name = [0u8 ; 16] ; OsRng . fill_bytes (& mut key_name) ; Ok (Box :: new (Self { key : ChaCha20Poly1305 :: new (& key) , key_name , maximum_ciphertext_len : AtomicUsize :: new (0) , })) } }
};
}
