// Generated macro for impl_78 (impl)
macro_rules! Depcrate_ed25519impl_78 {
() => {
// Module: crate::ed25519
// Provides: {"impl_78"}
// Dependencies: {}
impl PublicKey { # [doc = " Number of raw bytes in a public key."] pub const BYTES : usize = 32 ; # [doc = " Creates a public key from raw bytes."] pub fn new (pk : [u8 ; PublicKey :: BYTES]) -> Self { PublicKey (pk) } # [doc = " Creates a public key from a slice."] pub fn from_slice (pk : & [u8]) -> Result < Self , Error > { let mut pk_ = [0u8 ; PublicKey :: BYTES] ; if pk . len () != pk_ . len () { return Err (Error :: InvalidPublicKey) ; } pk_ . copy_from_slice (pk) ; Ok (PublicKey :: new (pk_)) } }
};
}
