// Generated macro for impl_97 (impl)
macro_rules! Depcrate_ed25519impl_97 {
() => {
// Module: crate::ed25519
// Provides: {"impl_97"}
// Dependencies: {}
impl PublicKey { # [doc = " Verify the signature of a multi-part message (streaming)."] pub fn verify_incremental (& self , signature : & Signature) -> Result < VerifyingState , Error > { VerifyingState :: new (self , signature) } # [doc = " Verifies that the signature `signature` is valid for the message"] # [doc = " `message`."] pub fn verify (& self , message : impl AsRef < [u8] > , signature : & Signature) -> Result < () , Error > { let mut st = VerifyingState :: new (self , signature) ? ; st . absorb (message) ; st . verify () } }
};
}
