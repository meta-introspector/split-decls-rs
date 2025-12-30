// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl Signer for Keypair { # [inline] fn pubkey (& self) -> Address { Address :: from (self . 0 . verifying_key () . to_bytes ()) } fn try_pubkey (& self) -> Result < Address , SignerError > { Ok (self . pubkey ()) } fn sign_message (& self , message : & [u8]) -> Signature { Signature :: from (self . 0 . sign (message) . to_bytes ()) } fn try_sign_message (& self , message : & [u8]) -> Result < Signature , SignerError > { Ok (self . sign_message (message)) } fn is_interactive (& self) -> bool { false } }
};
}
