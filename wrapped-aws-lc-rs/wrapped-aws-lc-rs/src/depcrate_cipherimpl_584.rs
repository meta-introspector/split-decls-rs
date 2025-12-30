// Generated macro for impl_584 (impl)
macro_rules! Depcrate_cipherimpl_584 {
() => {
// Module: crate::cipher
// Provides: {"impl_584"}
// Dependencies: {}
impl UnboundCipherKey { # [doc = " Constructs an [`UnboundCipherKey`]."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " * [`Unspecified`] if `key_bytes.len()` does not match the length required by `algorithm`."] pub fn new (algorithm : & 'static Algorithm , key_bytes : & [u8]) -> Result < Self , Unspecified > { let key_bytes = Buffer :: new (key_bytes . to_vec ()) ; Ok (UnboundCipherKey { algorithm , key_bytes , }) } # [inline] # [must_use] # [doc = " Returns the algorithm associated with this key."] pub fn algorithm (& self) -> & 'static Algorithm { self . algorithm } }
};
}
