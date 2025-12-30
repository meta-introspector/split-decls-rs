// Generated macro for impl_91 (impl)
macro_rules! Depcrate_ed25519impl_91 {
() => {
// Module: crate::ed25519
// Provides: {"impl_91"}
// Dependencies: {}
impl Signature { # [doc = " Number of raw bytes in a signature."] pub const BYTES : usize = 64 ; # [doc = " Creates a signature from raw bytes."] pub fn new (bytes : [u8 ; Signature :: BYTES]) -> Self { Signature (bytes) } # [doc = " Creates a signature key from a slice."] pub fn from_slice (signature : & [u8]) -> Result < Self , Error > { let mut signature_ = [0u8 ; Signature :: BYTES] ; if signature . len () != signature_ . len () { return Err (Error :: InvalidSignature) ; } signature_ . copy_from_slice (signature) ; Ok (Signature :: new (signature_)) } }
};
}
