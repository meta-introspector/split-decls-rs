// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl Signer for Presigner { fn try_pubkey (& self) -> Result < Pubkey , SignerError > { Ok (self . pubkey) } fn try_sign_message (& self , message : & [u8]) -> Result < Signature , SignerError > { if self . signature . verify (self . pubkey . as_ref () , message) { Ok (self . signature) } else { Err (PresignerError :: VerificationFailure . into ()) } } fn is_interactive (& self) -> bool { false } }
};
}
